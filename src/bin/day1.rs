use std::io;

fn main() {
    let mut increase_counter = 0;
    const FIRST_TAKEN: usize = 0;
    let list = read_ints();
    if let Some(first) = list.get(FIRST_TAKEN) {
        let mut previous = first;
        for l in list.iter().skip(FIRST_TAKEN + 1) {
            if previous < l {
                increase_counter += 1;
            }
            previous = l;
        }
    }

    println!("{}", increase_counter)
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
