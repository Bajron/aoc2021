use std::io;

fn main() {
    let list = read_ints();
    let mut increase_counter = 0;
    let mut previous: Option<i64> = None;

    for l in list {
        if let Some(prev) = previous {
            if prev < l {
                increase_counter+=1;
            }
        }
        previous = Some(l);
    }

    println!("{}", increase_counter)
}

fn read_ints() -> Vec<i64>{
    if let Ok(lines) = read_lines() {
        lines.filter_map(|x|x.ok()).filter_map(|x| x.parse::<i64>().ok()).collect()
    } else {
        Vec::new()
    }
}

fn read_lines() -> io::Result<io::Lines<io::BufReader<io::Stdin>>> {
    use io::BufRead;
    Ok(io::BufReader::new(io::stdin()).lines())
}
