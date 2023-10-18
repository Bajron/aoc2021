use aoc2021::input::*;
use std::collections::{HashMap, HashSet};

fn main() {
    let input = read_lines().ok().and_then(|lines| {
        let mut graph = HashMap::<String, HashSet<String>>::new();

        lines
            .filter_map(|l| l.ok())
            .filter_map(|s| match s.split("-").collect::<Vec<&str>>()[..] {
                [from, to] => Some((from.to_owned(), to.to_owned())),
                _ => None,
            })
            .for_each(|(f, t)| {
                if let Some(set) = graph.get_mut(&f) {
                    set.insert(t.clone());
                } else {
                    graph.insert(f.clone(), HashSet::from([t.clone()]));
                };

                if let Some(set) = graph.get_mut(&t) {
                    set.insert(f.clone());
                } else {
                    graph.insert(t.clone(), HashSet::from([f.clone()]));
                }
            });
        Some(graph)
    });

    if let Some(graph) = input {
        println!("{graph:?}");
        let mut visited = HashSet::<String>::new();
        let w = ways(&String::from("start"), &graph, &mut visited);
        println!("Ways {w}");

        let w2 = ways2(&String::from("start"), &graph, &mut visited, None);
        println!("Other ways {w2}");
    }
}

fn ways(
    current: &String,
    graph: &HashMap<String, HashSet<String>>,
    visited: &mut HashSet<String>,
) -> u32 {
    if current == "end" {
        return 1;
    }

    if current.chars().next().is_some_and(|x| x.is_lowercase()) {
        visited.insert(current.clone());
    }

    let mut w = 0;
    for neighbor in &graph[current] {
        if visited.contains(neighbor) {
            continue;
        }
        w += ways(neighbor, graph, visited);
    }

    visited.remove(current);

    return w;
}

fn ways2(
    current: &String,
    graph: &HashMap<String, HashSet<String>>,
    visited: &mut HashSet<String>,
    double: Option<&String>,
) -> u32 {
    if current == "end" {
        return 1;
    }

    if current.chars().next().is_some_and(|x| x.is_lowercase()) {
        visited.insert(current.clone());
    }

    let mut w = 0;
    for neighbor in &graph[current] {
        if neighbor == "start" {
            continue;
        }

        if visited.contains(neighbor) {
            if double.is_some() {
                continue;
            } else {
                w += ways2(neighbor, graph, visited, Some(neighbor));
            }
        } else {
            w += ways2(neighbor, graph, visited, double.clone());
        }
    }

    if !double.is_some_and(|d| d == current) {
        visited.remove(current);
    }

    return w;
}
