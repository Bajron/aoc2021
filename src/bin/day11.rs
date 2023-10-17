use std::collections::HashSet;

use aoc2021::input::*;

fn main() {
    let input = read_lines().ok().and_then(|lines| {
        let states = [String::from("X")]
            .into_iter()
            .chain(lines.filter_map(|l| l.ok()))
            .chain([String::from("X")].into_iter())
            .map(|s| {
                ['X']
                    .into_iter()
                    .chain(s.chars())
                    .chain(['X'].into_iter())
                    .filter_map(|c| match c {
                        'X' => Some(i8::MAX),
                        _ => c.to_string().parse::<i8>().ok(),
                    })
                    .collect::<Vec<i8>>()
            })
            .collect::<Vec<Vec<i8>>>();

        let row_length = if let Some(second) = states.iter().nth(2) {
            second.len()
        } else {
            1
        };

        let mut states = states;
        if let Some(first) = states.first_mut() {
            first.resize(row_length, i8::MAX);
        }
        if let Some(last) = states.last_mut() {
            last.resize(row_length, i8::MAX);
        }

        Some(states)
    });

    if let Some(states) = input {
        let height = states.len() - 2;
        let width = states
            .iter()
            .skip(1)
            .next()
            .and_then(|r| Some(r.len() - 2))
            .unwrap_or(0);

        println!("Width: {width} Height: {height}");

        let mut states = states;
        let mut flashed: Vec<Vec<bool>> = vec![vec![false; width + 2]; states.len()];

        let mut total_flashes = 0;

        let mut i = 0;
        loop {
            // println!("Day {i}");

            // for r in &states {
            //     println!("{r:?}");
            // }
            // println!();

            states
                .iter_mut()
                .flatten()
                .filter(|s| **s < i8::MAX)
                .for_each(|s| *s += 1);

            let mut flashing = (1..=height)
                .flat_map(|y| (1..=width).map(move |x| (y, x)))
                .filter(|&(y, x)| states[y][x] > 9)
                .collect::<HashSet<(usize, usize)>>();
            flashing.iter().for_each(|&(y, x)| flashed[y][x] = true);

            while flashing.len() > 0 {
                let to_increase: Vec<(usize, usize)> = flashing
                    .iter()
                    .flat_map(|&(y, x)| neigbours(y, x))
                    .filter(|&(y, x)| !flashed[y][x] && states[y][x] < i8::MAX)
                    .collect();
                to_increase.iter().for_each(|&(y, x)| states[y][x] += 1);

                flashing = to_increase
                    .into_iter()
                    .filter(|&(y, x)| !flashed[y][x] && states[y][x] > 9)
                    .collect();

                flashing.iter().for_each(|&(y, x)| flashed[y][x] = true);
            }

            let mut flash_count = 0;
            flashed
                .iter_mut()
                .flatten()
                .zip(states.iter_mut().flatten())
                .filter(|(f, _)| **f)
                .for_each(|(f, s)| {
                    *f = false;
                    *s = 0;
                    flash_count += 1;
                });
            // println!("Flashed {flash_count}");
            total_flashes += flash_count;
            i += 1;
            if i == 100 {
                println!("Flashes in 100 iterations: {total_flashes}");
            }
            if flash_count == 100 {
                println!("All flashes after {i} iterations");
                break;
            }
        }
    }
}

fn neigbours(y: usize, x: usize) -> [(usize, usize); 8] {
    return [
        (y - 1, x - 1),
        (y - 1, x),
        (y - 1, x + 1),
        (y, x - 1),
        // (y, x),
        (y, x + 1),
        (y + 1, x - 1),
        (y + 1, x),
        (y + 1, x + 1),
    ];
}
