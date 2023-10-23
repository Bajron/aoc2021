use aoc2021::input::*;

fn main() {
    let input = read_lines().ok().and_then(|lines| {
        let mut it = lines.filter_map(|l| l.ok()).into_iter();
        let itr = &mut it;

        let algo_line = itr.next();
        itr.next();

        let picture = itr.collect::<Vec<String>>();

        algo_line.and_then(|al| Some((al, picture)))
    });

    if let Some((algo_line, picture)) = input {
        println!("Algo: {algo_line}");
        let show = false;

        if &algo_line[0..1] != "." {
            println!("Algorithm generates points from emptyness!")
        }
        let alternating = &algo_line[0..1] == "#" && &algo_line[511..512] == ".";

        let algo_bits = algo_line
            .chars()
            .map(|c| if c == '.' { '0' } else { '1' })
            .collect::<String>();

        let height = picture.len();
        let width = picture
            .iter()
            .skip(1)
            .next()
            .and_then(|r| Some(r.len()))
            .unwrap_or(0);

        let mut picture = picture;
        picture.iter_mut().for_each(|s| {
            *s = s
                .chars()
                .map(|c| if c == '.' { '0' } else { '1' })
                .collect::<String>()
        });

        println!("Original width: {width} height: {height}");
        if show {
            for row in &picture {
                println!("{row}");
            }
        }

        let steps = 50;
        let step_margin = 1;
        let to_add = steps * step_margin + 1;

        let mut picture_extended: Vec<String> = vec![];
        for _ in 0..to_add {
            picture_extended.push("0".repeat(width + 2 * to_add));
        }
        for row in picture {
            picture_extended.push(
                std::iter::repeat('0')
                    .take(to_add)
                    .chain(row.chars())
                    .chain(std::iter::repeat('0').take(to_add))
                    .collect::<String>(),
            );
        }
        for _ in 0..to_add {
            picture_extended.push("0".repeat(width + 2 * to_add));
        }

        let orig_x = to_add;
        let orig_y = to_add;

        if show {
            println!();
            for row in &picture_extended {
                println!("{row}");
            }
        }

        let kernel_shift = 3 / 2;
        let mut next = picture_extended.clone();
        for s in 1..=steps {
            // There is one other case where we flip zeros to ones and they stay 1
            // but then we deal with inifinite number of ones, so such input
            // most likely will not appear ;)
            if alternating {
                let other = if &picture_extended[0][0..1] == "0" {
                    '1'
                } else {
                    '0'
                };
                let other_str = other.to_string().repeat(next[0].len());
                for row in next.iter_mut() {
                    row.replace_range(0..row.len(), &other_str);
                }
            }

            let start_x = orig_x - s - kernel_shift;
            let end_x = orig_x + width + s - kernel_shift;
            let start_y = orig_y - s - kernel_shift;
            let end_y = orig_y + height + s - kernel_shift;
            for y in start_y..end_y {
                for x in start_x..=end_x {
                    // y,x is the upper left corner of the kernel
                    let num = picture_extended[y]
                        .chars()
                        .skip(x)
                        .take(3)
                        .chain(picture_extended[y + 1].chars().skip(x).take(3))
                        .chain(picture_extended[y + 2].chars().skip(x).take(3))
                        .collect::<String>();
                    if let Some(n) = usize::from_str_radix(&num, 2).ok() {
                        next[y + 1].replace_range(x + 1..x + 2, &algo_bits[n..n + 1]);
                    }
                }
            }

            if show {
                println!();
                println!("After step {s}");
                for row in &next {
                    println!("{row}");
                }
            }

            std::mem::swap(&mut next, &mut picture_extended);
        }

        let ones = picture_extended
            .iter()
            .flat_map(|x| x.chars())
            .filter(|&c| c == '1')
            .count();
        println!("Steps {steps} completed");
        println!("Ones: {ones}");
        // 5176 too high, not handled bits from all empty...
    }
}
