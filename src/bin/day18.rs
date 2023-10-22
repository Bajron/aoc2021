use aoc2021::input::*;
use core::fmt;
use std::{cell::RefCell, rc::Rc, str::Chars};

#[derive(Debug)]
enum Node {
    Basic(Rc<RefCell<i32>>),
    Pair(Box<Node>, Box<Node>),
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Node::Basic(v) => write!(f, "{}", *v.borrow()),
            Node::Pair(l, r) => write!(f, "[{},{}]", *l, *r),
        }
    }
}

fn line_to_node(line: String) -> Option<Node> {
    read_node(line.chars().by_ref())
}

fn read_node(itr: &mut Chars<'_>) -> Option<Node> {
    while let Some(ch) = itr.next() {
        if ch == '[' {
            let left = read_node(itr);
            let right = read_node(itr);

            if let (Some(left), Some(right)) = (left, right) {
                return Some(Node::Pair(left.into(), right.into()));
            } else {
                return None;
            }
        } else if ch.is_numeric() {
            let mut buf = ch.to_string();
            itr.take_while(|c| c.is_numeric()).for_each(|c| buf.push(c));
            return buf
                .parse::<i32>()
                .ok()
                .and_then(|n| Some(Node::Basic(Rc::new(RefCell::new(n)))));
            // Here we stop with ']' or ',' read out
        }
        // ignore ',' or ']'; we are terrible at parsing, but only because we always consume the read out, right?
    }
    None
}

impl Node {
    fn reduce(&mut self) {
        // let mut i = 0;
        loop {
            // i += 1;
            if self.explode() {
                // println!("Explode: {}", self);
                continue;
            }
            if self.split() {
                // println!("Split:   {}", self);
                continue;
            }
            break;
        }
        // println!("Reduced after {i} steps");
    }

    fn explode(&mut self) -> bool {
        let mut previous_node: Option<Rc<RefCell<i32>>> = None;
        let mut value_for_next: Option<i32> = None;
        let mut any_exploded = false;

        self.explode_helper(
            0,
            &mut previous_node,
            &mut value_for_next,
            &mut any_exploded,
        );
        any_exploded
    }

    fn explode_helper(
        &mut self,
        depth: i32,
        prev: &mut Option<Rc<RefCell<i32>>>,
        next: &mut Option<i32>,
        any_exploded: &mut bool,
    ) -> bool {
        match self {
            Node::Pair(left, right) => {
                // Explode only once (there might be two exploding nodes on one level)
                if !*any_exploded {
                    // If it is a special case of deep Node of 2 integers, explode!
                    match (left.as_ref(), right.as_ref()) {
                        (Node::Basic(left), Node::Basic(right)) => {
                            if depth >= 4 {
                                if let Some(pv) = prev {
                                    *pv.borrow_mut() += *left.borrow();
                                }
                                *next = Some(*right.borrow());

                                *any_exploded = true;
                                return true;
                            }
                        }
                        _ => (),
                    }
                }
                // Any other pair

                let left_exploded = left.explode_helper(depth + 1, prev, next, any_exploded);
                if left_exploded {
                    *left = Box::new(Node::Basic(Rc::new(RefCell::new(0))));
                }
                if *any_exploded && next.is_none() {
                    // No need to go further right, it has exploded, and the value for the right is consumed
                    return false;
                }
                let right_exploded = right.explode_helper(depth + 1, prev, next, any_exploded);
                if right_exploded {
                    *right = Box::new(Node::Basic(Rc::new(RefCell::new(0))));
                }

                false
            }
            Node::Basic(v) => {
                if let Some(value) = next {
                    *v.borrow_mut() += *value;

                    *next = None;
                    // We can stop the walk
                }
                *prev = Some(v.clone());
                false
            }
        }
    }

    fn split(&mut self) -> bool {
        let mut any_split = false;
        self.split_helper(&mut any_split);
        any_split
    }

    fn split_helper(&mut self, any_split: &mut bool) -> Option<Node> {
        match self {
            Node::Pair(left, right) => {
                if let Some(split) = left.split_helper(any_split) {
                    *left = Box::new(split);
                }
                if *any_split {
                    return None;
                }
                if let Some(split) = right.split_helper(any_split) {
                    *right = Box::new(split);
                }
                None
            }
            Node::Basic(v) => {
                if *v.borrow() >= 10 {
                    *any_split = true;
                    let l = *v.borrow() / 2;
                    let r = (*v.borrow() + 1) / 2;

                    Some(Node::Pair(
                        Box::new(Node::Basic(Rc::new(RefCell::new(l)))),
                        Box::new(Node::Basic(Rc::new(RefCell::new(r)))),
                    ))
                } else {
                    None
                }
            }
        }
    }

    fn magnitude(&self) -> i32 {
        match self {
            Node::Pair(left, right) => 3 * left.magnitude() + 2 * right.magnitude(),
            Node::Basic(v) => *v.borrow(),
        }
    }
}

impl Clone for Node {
    fn clone(&self) -> Node {
        match self {
            Node::Pair(left, right) => Node::Pair(
                Box::new(left.as_ref().clone()),
                Box::new(right.as_ref().clone()),
            ),
            Node::Basic(v) => Node::Basic(Rc::new(RefCell::new(*v.borrow()))),
        }
    }
}

fn main() {
    let input = read_lines().ok().and_then(|lines| {
        let nodes = lines
            .into_iter()
            .filter_map(|l| l.ok())
            .filter_map(line_to_node)
            .collect::<Vec<Node>>();
        Some(nodes)
    });

    if let Some(nodes) = input {
        if let Some(current) = nodes.first() {
            let mut current = current.clone();

            println!("Initial: {}", current);
            current.reduce();
            println!("Reduced: {}", current);

            for node in nodes.iter().skip(1) {
                println!("Adding \n {}\n {} ", current, node);
                current = Node::Pair(Box::new(current), Box::new(node.clone()));
                current.reduce();
                println!("Got: {}", current);
            }
            println!("Final: {}", current);
            println!("Magnitude: {}", current.magnitude());
        }

        println!("Input has {} nodes", nodes.len());
        let mut max_magnitude = 0;
        for l in nodes.iter() {
            for r in nodes.iter() {
                if (format!("{l}") == format!("{r}")) {
                    continue;
                }

                let mut current = Node::Pair(Box::new(l.clone()), Box::new(r.clone()));
                current.reduce();

                max_magnitude = max_magnitude.max(current.magnitude());
            }
        }
        println!("Maximal magnitude for pairs {}", max_magnitude)
    }
}
