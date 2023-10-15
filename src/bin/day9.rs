use aoc2021::input::*;

fn main() {
    let input = read_lines().ok().and_then(|lines| {
        let height_map = [String::from("X")]
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

        let row_length = if let Some(second) = height_map.iter().nth(2) {
            second.len()
        } else {
            1
        };

        let mut height_map = height_map;
        if let Some(first) = height_map.first_mut() {
            first.resize(row_length, i8::MAX);
        }
        if let Some(last) = height_map.last_mut() {
            last.resize(row_length, i8::MAX);
        }

        Some(height_map)
    });

    if let Some(height_map) = input {
        let height = height_map.len() - 2;
        let width = height_map
            .iter()
            .skip(1)
            .next()
            .and_then(|r| Some(r.len() - 2))
            .unwrap_or(0);

        println!("Width: {width} Height: {height}");
        // for row in &height_map {
        //     println!("{row:?}");
        // }

        let is_low = |&(y, x)| -> bool {
            let hm = &height_map;
            neigbours(y, x)
                .iter()
                .all(|&(ny, nx)| hm[y][x] < hm[ny][nx])
        };

        let low_values = (1..=height)
            .flat_map(|y| (1..=width).map(move |x| (y, x)))
            // .filter(is_low) // EXPLAIN why u no work??
            .filter(|&(y, x)| -> bool {
                let hm = &height_map;
                neigbours(y, x)
                    .iter()
                    .all(|&(ny, nx)| hm[y][x] < hm[ny][nx])
            })
            .map(|(y, x)| height_map[y][x])
            .collect::<Vec<i8>>();

        println!("{low_values:?}");
        let score = low_values
            .iter()
            .map(|&x| x as i32 + 1)
            .fold(0, |acc, v| acc + v);

        println!("Score: {score}")
    }
}

fn neigbours(y: usize, x: usize) -> [(usize, usize); 4] {
    return [(y - 1, x), (y, x - 1), (y, x + 1), (y + 1, x)];
}
