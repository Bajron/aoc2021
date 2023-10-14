use std::io;

pub fn read_lines() -> io::Result<io::Lines<io::BufReader<io::Stdin>>> {
    use io::BufRead;
    Ok(io::BufReader::new(io::stdin()).lines())
}

pub fn comma_separated_to_vec(s: &str) -> Vec<i32> {
    s.trim()
        .split(",")
        .filter_map(|x| x.trim().parse::<i32>().ok())
        .collect()
}
