use aoc2021::input::*;
use std::collections::{HashMap, HashSet};

struct Entry {
    signals: Vec<String>,
    digits: Vec<String>,
}

type Display = [i8; 7];

fn main() {
    let input = read_lines().ok().and_then(|lines| {
        let entries = lines
            .filter_map(|x| x.ok())
            .filter_map(|l| match l.split("|").collect::<Vec<&str>>()[..] {
                [signals, digits] => Some(Entry {
                    signals: signals
                        .trim()
                        .split_whitespace()
                        .map(|x| x.to_string())
                        .collect(),
                    digits: digits
                        .trim()
                        .split_whitespace()
                        .map(|x| x.to_string())
                        .collect(),
                }),
                _ => {
                    eprintln!(" ** Line without separator");
                    None
                }
            })
            .collect::<Vec<Entry>>();
        Some(entries)
    });

    if let Some(entries) = input {
        let mut obvious_count = 0;
        let obvious_lengths: [usize; 4] = [2, 3, 4, 7];
        for e in entries.iter() {
            obvious_count += e
                .digits
                .iter()
                .filter(|x| obvious_lengths.contains(&(x.len())))
                .count();
        }

        println!("Obvious count: {}", obvious_count);

        let mut total = 0;
        for e in entries {
            let mapping = work_out_mapping(&e);
            assert!(mapping.len() == 7);

            let number = e
                .digits
                .iter()
                .map(|x| {
                    let mut display: Display = [0; 7];
                    x.chars()
                        .filter_map(|c| mapping.get(&c))
                        .for_each(|&i| display[i] = 1);
                    display
                })
                .map(|d| match d[..] {
                    [1, 1, 1, 0, 1, 1, 1] => 0,
                    [0, 0, 1, 0, 0, 1, 0] => 1,
                    [1, 0, 1, 1, 1, 0, 1] => 2,
                    [1, 0, 1, 1, 0, 1, 1] => 3,
                    [0, 1, 1, 1, 0, 1, 0] => 4,
                    [1, 1, 0, 1, 0, 1, 1] => 5,
                    [1, 1, 0, 1, 1, 1, 1] => 6,
                    [1, 0, 1, 0, 0, 1, 0] => 7,
                    [1, 1, 1, 1, 1, 1, 1] => 8,
                    [1, 1, 1, 1, 0, 1, 1] => 9,
                    _ => {
                        eprintln!("Invalid display {:?}", d);
                        0
                    }
                })
                .fold(0, |acc, n| acc * 10 + n);
            println!("The number is {}", number);
            total += number;
        }
        println!("Total is {total}")
    }
}

// '0' has 6
// '1' has 2
// '2' has 5
// '3' has 5
// '4' has 4
// '5' has 5
// '6' has 6
// '7' has 3
// '8' has 7
// '9' has 6
// intersect 0,6,9 to get 0,1,5,6

fn work_out_mapping(entry: &Entry) -> HashMap<char, usize> {
    //  0
    // 1 2
    //  3
    // 4 5
    //  6
    let mut wires = ['X'; 7];

    let digits_sets: Vec<HashSet<char>> = entry
        .signals
        .iter()
        .map(|v| v.chars().collect::<HashSet<char>>())
        .collect();

    let one = digits_sets.iter().filter(|f| f.len() == 2).next();
    let four = digits_sets.iter().filter(|f| f.len() == 4).next();
    let seven = digits_sets.iter().filter(|f| f.len() == 3).next();
    let eight = digits_sets.iter().filter(|f| f.len() == 7).next();

    if let (Some(one), Some(four), Some(seven), Some(eight)) = (one, four, seven, eight) {
        let top_bar = seven.difference(&one).copied().collect::<Vec<char>>();
        assert!(top_bar.len() == 1);
        wires[0] = top_bar[0];

        // intersect 0,6,9 to get 0,1,5,6
        let corners = digits_sets
            .iter()
            .filter(|x| x.len() == 6)
            .fold(eight.clone(), |acc, x| {
                acc.intersection(x).copied().collect::<HashSet<char>>()
            });
        assert!(corners.len() == 4);

        let bottom_bar = corners
            .difference(&seven)
            .copied()
            .collect::<HashSet<char>>()
            .difference(&four)
            .copied()
            .collect::<Vec<char>>();
        assert!(bottom_bar.len() == 1);
        wires[6] = bottom_bar[0];

        let bottom_right = corners.intersection(one).copied().collect::<Vec<char>>();
        assert!(bottom_right.len() == 1);
        wires[5] = bottom_right[0];

        let upper_right = one
            .difference(&HashSet::from([wires[5]]))
            .copied()
            .collect::<Vec<char>>();
        assert!(upper_right.len() == 1);
        wires[2] = upper_right[0];

        let mut known = HashSet::from([wires[0], wires[2], wires[5], wires[6]]);
        let three = digits_sets
            .iter()
            .filter(|x| x.difference(&known).count() == 1)
            .collect::<Vec<_>>();
        assert!(three.len() == 1);

        if let Some(three) = three.first() {
            let middle_bar = three.difference(&known).copied().collect::<Vec<char>>();
            assert!(middle_bar.len() == 1);
            wires[3] = middle_bar[0];
        }
        known.insert(wires[3]);

        let nine = digits_sets
            .iter()
            .filter(|x| x.len() == 6 && x.difference(&known).count() == 1)
            .collect::<Vec<_>>();
        assert!(nine.len() == 1);
        if let Some(nine) = nine.first() {
            let upper_left = nine.difference(&known).copied().collect::<Vec<char>>();
            assert!(upper_left.len() == 1);
            wires[1] = upper_left[0];
        }

        known.insert(wires[1]);

        let lower_left = eight.difference(&known).copied().collect::<Vec<char>>();
        assert!(lower_left.len() == 1);
        wires[4] = lower_left[0];

        // println!("{:?}", wires)
    }
    wires
        .iter()
        .enumerate()
        .map(|(i, &c)| (c, i))
        .collect::<HashMap<char, usize>>()
}
