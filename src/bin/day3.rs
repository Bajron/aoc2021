use aoc2021::input::read_lines;

fn main() {
    let lines: Vec<String> = read_lines()
        .ok()
        .and_then(|l| Some(l.filter_map(|x| x.ok()).collect()))
        .unwrap_or(Vec::new());

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
