use aoc2021::input::*;
use std::collections::HashMap;

fn main() {
    let input = read_lines().ok().and_then(|lines| {
        let mut it = lines.filter_map(|l| l.ok()).into_iter();
        let itr = &mut it;

        let mut scanners: HashMap<String, Vec<(i32, i32, i32)>> = HashMap::new();
        while let Some(l) = itr.next() {
            let name = l;
            let positions: Vec<(i32, i32, i32)> = itr
                .map(|l| comma_separated_to_vec(l.as_str()))
                .map_while(|v| match v[..] {
                    [x, y, z] => Some((x, y, z)),
                    _ => None,
                })
                .collect();
            scanners.insert(name, positions);
        }

        Some(scanners)
    });

    if let Some(scanners) = input {
        println!("{scanners:?}");
    }
}
