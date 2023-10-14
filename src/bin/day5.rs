use aoc2021::input::*;
use std::cmp::max;

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn is_horizontal(&self) -> bool {
        return self.start.y == self.end.y;
    }
    fn is_vertical(&self) -> bool {
        return self.start.x == self.end.x;
    }
    fn is_oblique(&self) -> bool {
        return !self.is_horizontal() && !self.is_vertical();
    }

    fn draw_x(&self) -> Box<dyn Iterator<Item = usize>> {
        let Line {
            start: Point { x: x1, y: _ },
            end: Point { x: x2, y: _ },
        } = self;
        let x1 = *x1 as usize;
        let x2 = *x2 as usize;
        if x1 < x2 {
            Box::new((x1..=x2).into_iter())
        } else {
            Box::new((x2..=x1).rev().into_iter())
        }
    }

    fn draw_y(&self) -> Box<dyn Iterator<Item = usize>> {
        let Line {
            start: Point { x: _, y: y1 },
            end: Point { x: _, y: y2 },
        } = self;
        let y1 = *y1 as usize;
        let y2 = *y2 as usize;
        if y1 < y2 {
            Box::new((y1..=y2).into_iter())
        } else {
            Box::new((y2..=y1).rev().into_iter())
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let input = read_lines().ok().and_then(|lines| {
        let lines_list: Vec<Line> = lines
            .filter_map(|l| l.ok())
            .filter_map(|l| match l.split(" -> ").collect::<Vec<_>>()[..] {
                [from, to] => {
                    let from = comma_separated_to_vec(from);
                    let to = comma_separated_to_vec(to);
                    match (&from[..], &to[..]) {
                        ([x1, y1], [x2, y2]) => Some(Line {
                            start: Point { x: *x1, y: *y1 },
                            end: Point { x: *x2, y: *y2 },
                        }),
                        _ => None,
                    }
                }
                _ => None,
            })
            .collect();

        Some(lines_list)
    });

    if let Some(lines) = input {
        println!("Read {} lines definitions", lines.len());

        let (max_x, max_y) = lines
            .iter()
            .flat_map(|l| [l.start, l.end])
            .fold((0, 0), |(mx, my), p| (max(mx, p.x), max(my, p.y)));

        avoiding_oblique(&lines, max_x, max_y);
        println!();
        not_avoiding_oblique(&lines, max_x, max_y);
    }
}

fn avoiding_oblique(lines: &Vec<Line>, max_x: i32, max_y: i32) {
    println!("Avoiding oblique lines");

    let mut board = vec![vec![0; max_x as usize + 1]; max_y as usize + 1];

    for line in lines.iter().filter(|l| !l.is_oblique()) {
        if line.is_horizontal() {
            for x in line.draw_x() {
                board[line.start.y as usize][x] += 1
            }
        }
        if line.is_vertical() {
            for y in line.draw_y() {
                board[y][line.start.x as usize] += 1
            }
        }
    }

    if max_x < 20 && max_y < 20 {
        for row in board.iter() {
            for c in row.iter() {
                if *c == 0 {
                    print!(".")
                } else {
                    print!("{c}")
                }
            }
            println!()
        }
    }

    let at_least2 = board.iter().flatten().filter(|&&x| x >= 2).count();
    println!("At least 2 times section for {} points", at_least2)
}

fn not_avoiding_oblique(lines: &Vec<Line>, max_x: i32, max_y: i32) {
    println!("Not avoiding oblique lines");

    let mut board = vec![vec![0; max_x as usize + 1]; max_y as usize + 1];

    for line in lines {
        if line.is_horizontal() {
            for x in line.draw_x() {
                board[line.start.y as usize][x] += 1
            }
        } else if line.is_vertical() {
            for y in line.draw_y() {
                board[y][line.start.x as usize] += 1
            }
        } else {
            for (y, x) in line.draw_y().zip(line.draw_x()) {
                board[y][x] += 1
            }
        }
    }

    if max_x < 20 && max_y < 20 {
        for row in board.iter() {
            for c in row.iter() {
                if *c == 0 {
                    print!(".")
                } else {
                    print!("{c}")
                }
            }
            println!()
        }
    }

    let at_least2 = board.iter().flatten().filter(|&&x| x >= 2).count();
    println!("At least 2 times section for {} points", at_least2)
}
