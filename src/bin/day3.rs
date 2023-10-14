use aoc2021::input::read_lines;

fn main() {
    let lines: Vec<String> = read_lines()
        .ok()
        .and_then(|l| Some(l.filter_map(|x| x.ok()).collect()))
        .unwrap_or(Vec::new());

    // TODO: the solution is probably much easier with sorting
    part1(&lines);
    println!();
    part2(&lines);
}

fn part1(lines: &Vec<String>) {
    if let Some(example) = lines.first() {
        let mut ones = vec![0; example.len()];
        let total = lines.len();
        for l in lines {
            for (i, ch) in l.chars().enumerate() {
                if ch == '1' {
                    ones[i] += 1
                }
            }
        }

        let gamma = ones
            .iter()
            .map(|x| (total - x, *x))
            .map(|(zeros, ones)| {
                if zeros == ones {
                    println!("** zeros equal to ones, ?!?");
                };
                if zeros > ones {
                    '0'
                } else {
                    '1'
                }
            })
            .collect::<String>();

        let epsilon = gamma
            .chars()
            .map(|c| if c == '0' { '1' } else { '0' })
            .collect::<String>();

        println!("Gamma: {gamma}");
        println!("Epsilon: {epsilon}");

        let g = u32::from_str_radix(gamma.as_str(), 2);
        let e = u32::from_str_radix(epsilon.as_str(), 2);

        if let (Ok(g), Ok(e)) = (g, e) {
            println!("{} x {} = {}", g, e, g * e);
        }
    }
}

fn part2(lines: &Vec<String>) {
    let bit_count = lines.first().and_then(|x| Some(x.len())).unwrap_or(0);
    println!("Working with {} lines.", lines.len());
    println!("Example bit length is {}", bit_count);

    let lines_o = bit_filter_to_single(lines, bit_count, '1');
    let o_binary = match &lines_o[..] {
        [single] => {
            println!("O left with {single}");
            Some(single)
        }
        _ => {
            println!(
                "Something went wrong, we have {} entries for O",
                lines_o.len()
            );
            None
        }
    };

    let lines_co2 = bit_filter_to_single(lines, bit_count, '0');
    let co2_binary = match &lines_co2[..] {
        [single] => {
            println!("CO2 left with {single}");
            Some(single)
        }
        _ => {
            println!(
                "Something went wrong, we have {} entries for CO2",
                lines_co2.len()
            );
            None
        }
    };

    let o = o_binary.and_then(|s| u32::from_str_radix(s, 2).ok());
    let co2 = co2_binary.and_then(|s| u32::from_str_radix(s, 2).ok());

    if let (Some(o), Some(co2)) = (o, co2) {
        println!("O is {o}");
        println!("CO2 is {co2}");
        println!("The value is {}", o * co2);
    } else {
        eprintln!("Something went wrong...");
    }
}

fn bit_filter_to_single(
    lines: &Vec<String>,
    bit_count: usize,
    more_ones_or_draw: char,
) -> Vec<String> {
    let mut lines = lines.clone();
    let mut position = 0_usize;
    let other_char = flip_char_bit(more_ones_or_draw);
    while position < bit_count && lines.len() > 1 {
        let total = lines.len();
        let ones = one_count_at(&lines, position);
        let zeros = total - ones;
        let decider = if ones == zeros || ones > zeros {
            more_ones_or_draw
        } else {
            other_char
        };

        lines = lines
            .into_iter()
            .filter(|l| l.chars().nth(position).is_some_and(|c| c == decider))
            .collect();

        position += 1;
    }
    lines
}

fn flip_char_bit(ch: char) -> char {
    if ch == '0' {
        '1'
    } else {
        '0'
    }
}

fn one_count_at(lines: &Vec<String>, i: usize) -> usize {
    lines
        .iter()
        .filter_map(|x| x.chars().nth(i))
        .filter(|c| *c == '1')
        .count()
}
