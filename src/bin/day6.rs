use aoc2021::input::*;

fn main() {
    let input = read_lines().ok().and_then(|lines| {
        lines.take(1).last().and_then(|x| {
            x.ok()
                .and_then(|s| Some(comma_separated_to_vec(s.as_str())))
        })
    });

    if let Some(state) = input {
        simulate18(&state);

        let mut at_count = vec![0i64; 9];
        for s in state {
            at_count[s as usize] += 1
        }
        let mut previous = vec![0i64; 9];

        println!("Counts: {:?}", at_count);

        const DAYS: usize = 256;
        for day in 1..=DAYS {
            previous.copy_from_slice(&at_count);
            previous.rotate_left(1);
            at_count
                .iter_mut()
                .zip(previous.iter())
                .for_each(|(l, &r)| *l = r);
            at_count[6] += at_count[8];

            if day == 18 {
                println!("Fish after {} days: {}", day, at_count.iter().sum::<i64>())
            }
            if day == 80 {
                println!("Fish after {} days: {}", day, at_count.iter().sum::<i64>())
            }
        }
        println!("Fish after {} days: {}", DAYS, at_count.iter().sum::<i64>())
    }
}

fn simulate18(state: &Vec<i32>) {
    const DAYS: usize = 18;
    let mut state = state.clone();
    println!("Initial pool size: {}, {state:?}", state.len());
    for _ in 0..DAYS {
        let mut zeroed = 0;
        state.iter_mut().for_each(|d| {
            if *d == 0 {
                *d = 6;
                zeroed += 1;
            } else {
                *d -= 1;
            }
        });
        state.append(vec![8; zeroed].as_mut());
        println!("Pool size: {}, {state:?}", state.len())
    }
}
