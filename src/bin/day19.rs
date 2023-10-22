use aoc2021::input::*;
use std::collections::{HashMap, HashSet};

type Point = (i32, i32, i32);

fn reorient((x, y, z): Point, t: i32) -> Point {
    match t {
        // x facing right
        0 => (x, y, z),
        1 => (x, z, -y),
        2 => (x, -y, -z),
        3 => (x, -z, y),
        // x facing far
        4 => (-z, y, x),
        5 => (-y, -z, x),
        6 => (z, -y, x),
        7 => (y, z, x),
        // x facing left
        8 => (-x, y, -z),
        9 => (-x, -z, -y),
        10 => (-x, -y, z),
        11 => (-x, z, y),
        // x facing near
        12 => (z, y, -x),
        13 => (-y, z, -x),
        14 => (-z, -y, -x),
        15 => (y, -z, -x),
        // x facing up
        16 => (z, x, y),
        17 => (-y, x, z),
        18 => (-z, x, -y),
        19 => (y, x, -z),
        // x facing down
        20 => (y, -x, z),
        21 => (-z, -x, y),
        22 => (-y, -x, -z),
        23 => (z, -x, -y),
        _ => panic!("bad reorientation"),
    }
}

fn main() {
    let input = read_lines().ok().and_then(|lines| {
        let mut it = lines.filter_map(|l| l.ok()).into_iter();
        let itr = &mut it;

        let mut scanners: HashMap<String, HashSet<Point>> = HashMap::new();
        while let Some(l) = itr.next() {
            let name = l;
            let positions: HashSet<Point> = itr
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
        let names: Vec<String> = scanners.keys().into_iter().cloned().collect();

        let mut scanners_aligned: HashMap<String, HashSet<Point>> = HashMap::new();
        let mut to_align: HashSet<String> = names.iter().cloned().collect();

        let mut scanners_too_far: HashMap<String, HashSet<String>> = HashMap::new();
        let mut scanner_alignment: HashMap<String, (i32, Point)> = HashMap::new();

        scanners_aligned.insert(names[0].clone(), scanners[&names[0]].clone());
        to_align.remove(&names[0]);
        scanner_alignment.insert(names[0].clone(), (0, (0, 0, 0)));

        println!("Reference scanner is {}", names[0]);
        println!("{} scanners to align", to_align.len());

        while !to_align.is_empty() {
            let mut matching_key: Option<(String, i32, Point)> = None;
            for (current, set_a) in &scanners_aligned {
                for k in &to_align {
                    if scanners_too_far
                        .entry(current.clone())
                        .or_default()
                        .contains(k)
                    {
                        continue;
                    }
                    let set_b = &scanners[k];
                    if let Some((reo, translation)) = matching(set_a, set_b) {
                        println!("{current} is matching {k}");
                        matching_key = Some((k.clone(), reo, translation));
                        break;
                    } else {
                        scanners_too_far
                            .entry(current.clone())
                            .or_default()
                            .insert(k.clone());
                        scanners_too_far
                            .entry(k.clone())
                            .or_default()
                            .insert(current.clone());
                    }
                }
                if matching_key.is_some() {
                    break;
                }
            }
            if let Some((matching_key, reo, translation)) = matching_key {
                to_align.remove(&matching_key);
                scanner_alignment.insert(matching_key.clone(), (reo, translation));
                scanners_aligned.insert(
                    matching_key.clone(),
                    reorient_and_translate(&scanners[&matching_key], reo, &translation),
                );
            }
        }
        let point = scanners_aligned
            .values()
            .flatten()
            .copied()
            .collect::<HashSet<Point>>();
        println!("Beacons {}", point.len());

        let mut largest_distance = 0;
        for (_, t_a) in scanner_alignment.values() {
            for (_, t_b) in scanner_alignment.values() {
                let d = man_dist(t_a, t_b);
                largest_distance = largest_distance.max(d);
            }
        }
        println!("{scanner_alignment:?}");
        println!("Largest distance is {largest_distance}")
    }
}

fn man_dist(lhs: &Point, rhs: &Point) -> u32 {
    lhs.0.abs_diff(rhs.0) + lhs.1.abs_diff(rhs.1) + lhs.2.abs_diff(rhs.2)
}

fn sub(lhs: &Point, rhs: &Point) -> Point {
    return (lhs.0 - rhs.0, lhs.1 - rhs.1, lhs.2 - rhs.2);
}

fn reorient_and_translate(
    set: &HashSet<Point>,
    reorientation: i32,
    translation: &Point,
) -> HashSet<Point> {
    set.iter()
        .map(|p| sub(&reorient(*p, reorientation), translation))
        .collect()
}

fn matching(a_set: &HashSet<Point>, b_set: &HashSet<Point>) -> Option<(i32, Point)> {
    for r in 0..24 {
        for a in a_set {
            for b in b_set {
                let bb = reorient(*b, r);
                let translation = sub(&bb, a);
                // Assume a matches b after reorientation
                let processed = reorient_and_translate(b_set, r, &translation);
                if a_set.intersection(&processed).count() >= 12 {
                    return Some((r, translation));
                }
            }
        }
    }
    return None;
}
