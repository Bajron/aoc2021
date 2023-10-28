use aoc2021::input::*;

fn main() {
    let input = read_lines().ok().and_then(|lines| {
        let map = lines
            .into_iter()
            .filter_map(|l| l.ok())
            .map(|s| s.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        Some(map)
    });

    if let Some(map) = input {
        let height = map.len();
        let width = map.iter().next().and_then(|r| Some(r.len())).unwrap_or(0);

        println!("Width: {width}");
        println!("Height: {height}");

        let mut step = 0;
        let mut current = map.clone();
        let mut next;
        loop {
            let mut moved = 0;

            next = current.clone(); // TODO, save some allocations...
            for y in 0..height {
                for x in 0..width {
                    if current[y][x] == '>' {
                        let new_x = (x + 1) % width;
                        if current[y][new_x] == '.' {
                            next[y][x] = '.';
                            next[y][new_x] = '>';
                            moved += 1;
                        }
                    }
                }
            }
            std::mem::swap(&mut next, &mut current);
            next = current.clone();
            for y in 0..height {
                for x in 0..width {
                    if current[y][x] == 'v' {
                        let new_y = (y + 1) % height;
                        if current[new_y][x] == '.' {
                            next[y][x] = '.';
                            next[new_y][x] = 'v';
                            moved += 1;
                        }
                    }
                }
            }
            std::mem::swap(&mut next, &mut current);

            step += 1;
            if moved == 0 {
                break;
            }
        }
        println!("No move in {step}");
    }
}
