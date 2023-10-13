use aoc2021::input::read_lines;

struct Command(&'static str, i32);
fn main() {
    let lines = read_lines();

    if let Ok(lines) = lines {
        let commands = lines
            .filter_map(|x| x.ok())
            .map(|x| {
                x.split_whitespace()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
            })
            .filter(|x| x.len() == 2)
            .map(|x| (x[0].to_owned(), x[1].parse::<i32>()))
            .filter_map(|x| match x {
                (s, Ok(i)) => match s.as_str() {
                    "forward" => Some(Command("forward", i)),
                    "down" => Some(Command("down", i)),
                    "up" => Some(Command("up", i)),
                    _ => None,
                },
                _ => None,
            })
            .collect();

        interpret1(&commands);
        println!();
        interpret2(&commands);
    }
}

fn interpret1(commands: &Vec<Command>) {
    let mut depth = 0;
    let mut position = 0;
    for command in commands {
        match command.0 {
            "forward" => position += command.1,
            "down" => depth += command.1,
            "up" => depth -= command.1,
            _ => panic!("unkown command"),
        }
    }

    println!("Depth: {depth}");
    println!("Position: {position}");

    println!("Result#1: {}", depth * position)
}

fn interpret2(commands: &Vec<Command>) {
    let mut depth = 0;
    let mut position = 0;
    let mut aim = 0;
    for command in commands {
        match command.0 {
            "forward" => {
                position += command.1;
                depth += aim * command.1
            }
            "down" => aim += command.1,
            "up" => aim -= command.1,
            _ => panic!("unkown command"),
        }
    }

    println!("Depth: {depth}");
    println!("Position: {position}");
    println!("Aim: {aim}");

    println!("Result#2: {}", depth * position)
}
