use aoc2021::input::*;

fn main() {
    let input = read_lines().ok().and_then(|lines| {
        let map = lines
            .filter_map(|l| l.ok())
            .map(|s| {
                s.chars()
                    .filter_map(|c| c.to_digit(10))
                    .map(|u| u as u8)
                    .collect::<Vec<u8>>()
            })
            .collect::<Vec<Vec<u8>>>();
        Some(map)
    });

    if let Some(map) = input {
        let width = map.iter().next().and_then(|x| Some(x.len())).unwrap_or(0);
        let height = map.len();

        let mut big_map = vec![vec![0_u8; 5 * width]; 5 * height];

        for yy in 0..height {
            for xx in 0..width {
                for y in 0..5_u8 {
                    for x in 0..5_u8 {
                        big_map[yy + y as usize * height][xx + x as usize * width] =
                            (map[yy][xx] + y + x - 1) % 9 + 1;
                    }
                }
            }
        }

        analyse_map(big_map);
        analyse_map(map);
    }
}

fn analyse_map(map: Vec<Vec<u8>>) {
    let width = map.iter().next().and_then(|x| Some(x.len())).unwrap_or(0);
    let height = map.len();

    let neighbors = |(y, x): (usize, usize)| {
        let ys = if y == 0 {
            [y + 1, usize::MAX]
        } else if y >= height - 1 {
            [y - 1, usize::MAX]
        } else {
            [y - 1, y + 1]
        };
        let xs = if x == 0 {
            [x + 1, usize::MAX]
        } else if x >= width - 1 {
            [x - 1, usize::MAX]
        } else {
            [x - 1, x + 1]
        };
        ys.iter()
            .filter(|&&y| y != usize::MAX)
            .map(|y| (*y, x))
            .chain(xs.iter().filter(|&&x| x != usize::MAX).map(|x| (y, *x)))
            .collect::<Vec<(usize, usize)>>()
    };

    println!("{:?}", neighbors((1, 1)));
    println!("{:?}", neighbors((0, 1)));
    println!("{:?}", neighbors((0, 0)));
    println!("{:?}", neighbors((height - 1, width - 1)));

    let mut shortest = vec![vec![usize::MAX; width]; height];
    shortest[0][0] = 0;

    for _ in 0..15 {
        for y in 0..height {
            for x in 0..width {
                for n in neighbors((y, x)) {
                    if shortest[n.0][n.1] == usize::MAX {
                        continue;
                    }
                    shortest[y][x] = shortest[y][x].min(map[y][x] as usize + shortest[n.0][n.1]);
                }
            }
        }
        println!("Approx {}", shortest[height - 1][width - 1]);
    }

    println!("Pre check done");

    dfs(0, (0, 0), &map, &mut shortest, &neighbors, &|(y, x)| {
        y == height - 1 && x == width - 1
    });
    let score = shortest[height - 1][width - 1];
    println!("Best {score}")
}

fn dfs<F, G>(
    value: usize,
    pos: (usize, usize),
    map: &Vec<Vec<u8>>,
    shortest: &mut Vec<Vec<usize>>,
    neighbours: &F,
    is_finish: &G,
) where
    F: Fn((usize, usize)) -> Vec<(usize, usize)>,
    G: Fn((usize, usize)) -> bool,
{
    if value >= shortest[pos.0][pos.1] {
        return;
    }

    shortest[pos.0][pos.1] = value;

    if is_finish(pos) {
        println!("Finish {}", value);
        return;
    }

    for n in neighbours(pos) {
        dfs(
            value + map[n.0][n.1] as usize,
            n,
            map,
            shortest,
            neighbours,
            is_finish,
        );
    }
}
