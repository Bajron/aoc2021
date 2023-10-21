use aoc2021::input::*;

fn main() {
    let input: Option<((i32, i32), (i32, i32))> = read_lines().ok().and_then(|lines| {
        if let Some(first) = lines.into_iter().find_map(|l| l.ok()) {
            match first
                .replace("target area: ", "")
                .split(", ")
                .collect::<Vec<&str>>()[..]
            {
                [for_x, for_y] => {
                    let xr = get_range(for_x.replace("x=", "").as_str());
                    let yr = get_range(for_y.replace("y=", "").as_str());
                    println!("{xr:?} {yr:?}");
                    if let (Some(xr), Some(yr)) = (xr, yr) {
                        Some((xr, yr))
                    } else {
                        None
                    }
                }
                _ => None,
            }
        } else {
            None
        }
    });

    if let Some((xlimit, ylimit)) = input {
        let in_target =
            |(x, y): (i32, i32)| xlimit.0 <= x && x <= xlimit.1 && ylimit.0 <= y && y <= ylimit.1;

        println!("x: {xlimit:?}, y: {ylimit:?}");

        // Assume for now area is in x+ y- area (it is)
        let mut hits: Vec<((i32, i32), i32)> = vec![];

        for x in 1..(xlimit.1 + 10) {
            for y in (ylimit.0 - 10)..400 {
                let initial_v = (x, y);

                let mut v = initial_v.clone();
                let mut pos = (0, 0);
                let mut best_y = 0;
                loop {
                    // println!("pos {pos:?}");
                    if pos.0 < xlimit.0 && v.0 <= 0 {
                        break;
                    }
                    if pos.0 > xlimit.1 && v.0 >= 0 {
                        break;
                    }
                    if pos.1 < ylimit.0 && v.1 <= 0 {
                        break;
                    }

                    best_y = best_y.max(pos.1);
                    if in_target(pos) {
                        hits.push((initial_v, best_y));
                        break;
                    }

                    pos = (pos.0 + v.0, pos.1 + v.1);
                    v = (
                        if v.0 > 0 {
                            v.0 - 1
                        } else if v.0 < 0 {
                            v.0 + 1
                        } else {
                            v.0
                        },
                        v.1 - 1,
                    );
                }
            }
        }

        println!("Hits: {hits:?}");
        println!("Hit count: {}", hits.len());
        println!("Max: {}", hits.iter().map(|(_, my)| my).max().unwrap());
    }
}

fn get_range(range_string: &str) -> Option<(i32, i32)> {
    match range_string.split("..").collect::<Vec<&str>>()[..] {
        [x_from, x_to] => x_from
            .parse::<i32>()
            .ok()
            .and_then(|f| x_to.parse::<i32>().ok().and_then(|t| Some((f, t)))),
        _ => None,
    }
}
