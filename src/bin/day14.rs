use std::{collections::HashMap, mem::swap};

use aoc2021::input::read_lines;

fn main() {
    let input = read_lines().ok().and_then(|lines| {
        let mut it = lines.filter_map(|l| l.ok());

        let template = it.by_ref().next().unwrap_or(String::new());
        let rules = it
            .filter_map(|x| {
                x.split_once(" -> ")
                    .map(|x| (x.0.to_owned(), x.1.chars().next().unwrap()))
            })
            .collect::<HashMap<String, char>>();

        Some((template, rules))
    });

    if let Some((template, rules)) = input {
        println!("Rules: {rules:?}");
        println!("Template: {template}");

        let iterations = 40;

        let result = process_string(&template, &rules, iterations.min(10));
        analyse_string(result);

        let mut pairs: HashMap<String, usize> = HashMap::new();
        for i in 0..(template.len() - 1) {
            let considered = &template[i..i + 2];
            *pairs.entry(considered.to_string()).or_default() += 1;
        }

        let mut next_pairs: HashMap<String, usize> = HashMap::new();
        for _ in 0..iterations {
            next_pairs.clear();
            pairs
                .iter()
                .flat_map(|(p, &c)| {
                    if let Some(v) = rules.get(p) {
                        let mut left = p.clone();
                        left.replace_range(0..1, &v.to_string());
                        let mut right = p.clone();
                        right.replace_range(1..2, &v.to_string());
                        [(left, c), (right, c)]
                    } else {
                        [(p.clone(), c), (p.clone(), 0)]
                    }
                })
                .for_each(|(ch, count)| *next_pairs.entry(ch).or_default() += count);

            swap(&mut next_pairs, &mut pairs);
        }

        let mut counts: HashMap<char, usize> = HashMap::new();
        pairs.iter().for_each(|(p, count)| {
            p.chars().for_each(|c| {
                *counts.entry(c).or_default() += count;
            })
        });
        // All but first and last letter is counted twice
        println!("Counts raw: {counts:?}");

        let first = template.chars().next().unwrap();
        let last = template.chars().last().unwrap();
        println!("First {first}, last {last}");
        *counts.entry(first).or_default() += 1;
        *counts.entry(last).or_default() += 1;
        // println!("Counts raw: {counts:?}");

        counts.iter_mut().for_each(|(_, v)| {
            *v /= 2;
        });

        // 2304722022017 is too low
        analyse_counts(counts)
    }
}

fn analyse_string(result: String) {
    println!("Lenght {}", result.len());

    let mut counts: HashMap<char, usize> = HashMap::new();
    result.chars().for_each(|c| {
        *counts.entry(c).or_default() += 1;
    });

    analyse_counts(counts);
}

fn analyse_counts(counts: HashMap<char, usize>) {
    println!("Counts: {counts:?}");

    let (min, max) = counts.iter().fold((usize::MAX, 0), |(min, max), (_, c)| {
        (min.min(*c), max.max(*c))
    });

    println!("{max} - {min} = {}", max - min);
}

fn process_string(template: &String, rules: &HashMap<String, char>, iterations: i32) -> String {
    let mut prev_string = template.clone();
    let mut next_string = String::new();
    for i in 0..iterations {
        next_string.clear();
        for i in 0..(prev_string.len() - 1) {
            let considered = &prev_string[i..i + 2];
            next_string.push(considered.chars().next().unwrap());
            if let Some(v) = rules.get(considered) {
                next_string.push(*v);
            }
        }
        next_string.push(prev_string.chars().last().unwrap());

        if next_string.len() < 200 {
            println!("After step {}: {}", i + 1, next_string);
        } else {
            println!("After step {}: length {}", i + 1, next_string.len());
        }
        swap(&mut prev_string, &mut next_string);
    }
    prev_string
}
