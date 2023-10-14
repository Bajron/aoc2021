use aoc2021::input::*;
use std::collections::{HashMap, HashSet};

fn main() {
    let input = read_lines().ok().and_then(|lines| {
        lines.take(1).last().and_then(|x| {
            x.ok()
                .and_then(|s| Some(comma_separated_to_vec(s.as_str())))
        })
    });

    let simple = |d: u32| d;
    let increasing = |d: u32| d * (d + 1) / 2;

    if let Some(positions) = input {
        macro_rules! call {
            ($search_fun:ident, $cost_fun:ident) => {
                $search_fun(&positions, $cost_fun, stringify!($cost_fun))
            };
        }

        call!(find_lowest_cost_values_only, simple);
        call!(find_lowest_cost_all, simple);

        call!(find_lowest_cost_all, increasing);
    }
}

fn find_lowest_cost_all(positions: &Vec<i32>, cost_fun: impl Fn(u32) -> u32, info: &str) {
    let (min, max) = positions
        .iter()
        .fold((i32::MAX, i32::MIN), |(min, max), &x| {
            (std::cmp::min(min, x), std::cmp::max(max, x))
        });

    let costs = get_costs(Box::new((min..max).into_iter()), positions, cost_fun);

    let (min, cost) = get_min_cost(costs);

    println!("Minimal for {min}, cost ({info}) is {cost}. Searched all values in range.")
}


fn find_lowest_cost_values_only(positions: &Vec<i32>, cost_fun: impl Fn(u32) -> u32, info: &str) {
    let values: HashSet<i32> = positions.iter().copied().collect();
    let costs = get_costs(Box::new(values.into_iter()), positions, cost_fun);

    let (min, cost) = get_min_cost(costs);

    println!("Minimal for {min}, cost ({info}) is {cost}. Searched only provided values.")
}

fn get_costs(iter: Box<dyn Iterator<Item=i32>>, positions: &Vec<i32>, cost_fun: impl Fn(u32) -> u32) -> HashMap<i32, u32> {
    let mut costs: HashMap<i32, u32> = HashMap::new();
    for v in iter {
        let cost: u32 = positions.iter().map(|x| cost_fun(x.abs_diff(v))).sum();
        costs.insert(v, cost);
    }
    costs
}

fn get_min_cost(costs: std::collections::HashMap<i32, u32>) -> (i32, u32) {
    costs
        .iter()
        .fold((0, u32::MAX), |(min, cost), (&x, &x_cost)| {
            if x_cost < cost {
                (x, x_cost)
            } else {
                (min, cost)
            }
        })
}
