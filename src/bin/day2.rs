use aoc2021::input::read_lines;

enum Order {
    FORWARD,
    DOWN,
    UP,
}
struct Command(Order, i32);
fn main() {
    let lines = read_lines();

    if let Ok(lines) = lines {
        let commands = lines
            .filter_map(|x| x.ok())
            .filter_map(|x| {
                let parts = x.split_whitespace().collect::<Vec<&str>>();
                match parts[..] {
                    [name, value] => match name {
                        "forward" => Some((Order::FORWARD, value.parse::<i32>())),
                        "down" => Some((Order::DOWN, value.parse::<i32>())),
                        "up" => Some((Order::UP, value.parse::<i32>())),
                        _ => {eprintln!(" ** Unkown command {}", name); None},
                    },
                    _ => None,
                }
                .and_then(|p| match p {
                    (command, Ok(value)) => Some(Command(command, value)),
                    _ => {eprintln!(" ** String parsing error: {:?}", p.1); None},
                })
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
    for Command(order, value) in commands {
        match order {
            Order::FORWARD => position += value,
            Order::DOWN => depth += value,
            Order::UP => depth -= value,
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
    for Command(order, value) in commands {
        match order {
            Order::FORWARD => {
                position += value;
                depth += aim * value
            }
            Order::DOWN => aim += value,
            Order::UP => aim -= value,
        }
    }

    println!("Depth: {depth}");
    println!("Position: {position}");
    println!("Aim: {aim}");

    println!("Result#2: {}", depth * position)
}
