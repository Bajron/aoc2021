
use std::io;
pub fn read_lines() -> io::Result<io::Lines<io::BufReader<io::Stdin>>> {
    use io::BufRead;
    Ok(io::BufReader::new(io::stdin()).lines())
}
