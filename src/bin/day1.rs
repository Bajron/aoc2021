use std::io;

fn main() {
    let list = read_ints();

    // musings(&list);

    // The Solution
    for window in 1..=5 {
        let mut increased = 0;
        for (previous, current) in list.iter().zip(list.iter().skip(window)) {
            if current > previous {
                increased += 1;
            }
        }
        println!("Window {}, increased {}", window, increased);
    }
}

fn read_ints() -> Vec<i64> {
    if let Ok(lines) = read_lines() {
        lines
            .filter_map(|x| x.ok())
            .filter_map(|x| x.parse::<i64>().ok())
            .collect()
    } else {
        Vec::new()
    }
}

fn read_lines() -> io::Result<io::Lines<io::BufReader<io::Stdin>>> {
    use io::BufRead;
    Ok(io::BufReader::new(io::stdin()).lines())
}

#[allow(dead_code)]
fn musings(list: &Vec<i64>) {
    let mut increase_counter = 0;
    const FIRST_TAKEN: usize = 0;
    if let Some(first) = list.get(FIRST_TAKEN) {
        let mut previous = first;
        for current in list.iter().skip(FIRST_TAKEN + 1) {
            if previous < current {
                increase_counter += 1;
            }
            previous = current;
        }
    }
    println!("Increase 1: {}", increase_counter);

    // MapWindows could be something, but it is experimental for now.
    // https://doc.rust-lang.org/std/iter/struct.MapWindows.html
    let mut increase_window_counter = 0;
    const WINDOW: usize = 3;
    if list.len() > WINDOW {
        let mut previous = list.iter().take(3).fold(0, |x, y| x + y);
        for i in 0..list.len() - WINDOW {
            let current = previous - list[i] + list[i + WINDOW];
            if previous < current {
                increase_window_counter += 1;
            }
            previous = current;
        }
    }
    println!("Increase window {}: {}", WINDOW, increase_window_counter);

    increase_window_counter = 0;
    if list.len() > WINDOW {
        for i in 0..list.len() - WINDOW {
            let current_minus_previous = list[i + WINDOW] - list[i];
            if current_minus_previous > 0 {
                increase_window_counter += 1;
            }
        }
    }
    println!("Increase window {}: {}", WINDOW, increase_window_counter);

    increase_window_counter = 0;
    for (previous, current) in list.iter().zip(list.iter().skip(WINDOW)) {
        if current > previous {
            increase_window_counter += 1;
        }
    }
    println!("Increase window {}: {}", WINDOW, increase_window_counter);

    let x = list.iter().skip(list.len() + 2).fold(0, |x, y| x + y);
    println!("{} skipped?", x);
}
