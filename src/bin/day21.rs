use std::collections::HashMap;

use aoc2021::input::*;

struct Die {
    next: i32,
    rolls: i32,
}

impl Die {
    fn new() -> Die {
        return Die { next: 1, rolls: 0 };
    }

    fn roll(&mut self) -> i32 {
        let to_return = self.next;
        self.rolls += 1;
        self.next += 1;
        if self.next > 100 {
            self.next = 1;
        }
        to_return
    }
}

#[derive(Copy, Clone, Debug)]
struct MultiPlayer {
    p1_position: i32,
    p1_score: i32,
    p2_position: i32,
    p2_score: i32,

    worlds: u64,
}

fn main() {
    let input = read_lines().ok().and_then(|lines| {
        let players = lines
            .filter_map(|l| l.ok())
            .filter_map(
                |x| match &x.split(" starting position: ").collect::<Vec<&str>>()[..] {
                    [player, start] => start
                        .parse::<i32>()
                        .ok()
                        .and_then(|n| Some((player.to_string(), n))),
                    _ => None,
                },
            )
            .collect::<Vec<(String, i32)>>();
        Some(players)
    });

    if let Some(players) = input {
        if let [p1, p2] = &players[0..2] {
            let mut die = Die::new();

            let mut p1_pos = p1.1;
            let mut p2_pos = p2.1;

            let mut p1_score = 0;
            let mut p2_score = 0;

            println!("P1 starts at {p1_pos}");
            println!("P2 starts at {p2_pos}");

            while p1_score < 1000 && p2_score < 1000 {
                let p1_move = die.roll() + die.roll() + die.roll();
                p1_pos = advance(p1_pos, p1_move);
                p1_score += p1_pos;
                if p1_score >= 1000 {
                    break;
                }

                let p2_move = die.roll() + die.roll() + die.roll();
                p2_pos = advance(p2_pos, p2_move);
                p2_score += p2_pos;
            }

            println!("P1 score {p1_score}");
            println!("P2 score {p2_score}");
            println!("Die rolls {}", die.rolls);

            let check = p2_score.min(p1_score) * die.rolls;
            println!("Check: {check}");

            let mut counts: HashMap<i32, u64> = HashMap::new();
            for i in 1..=3 {
                for j in 1..=3 {
                    for k in 1..=3 {
                        let v = i + j + k;
                        *counts.entry(v).or_default() += 1;
                    }
                }
            }
            let counts = counts;
            println!("{counts:?}");

            let mut states: Vec<MultiPlayer> = vec![];
            let mut next: Vec<MultiPlayer> = vec![];
            states.push(MultiPlayer {
                p1_position: p1.1,
                p1_score: 0,
                p2_position: p2.1,
                p2_score: 0,
                worlds: 1,
            });

            let mut p1_wins = 0_u64;
            let mut p2_wins = 0_u64;

            while !states.is_empty() {
                println!("P1 moves, {} states", states.len());

                next.clear();
                for s in states.iter() {
                    for (change, count) in &counts {
                        let pos = advance(s.p1_position, *change);
                        let sco = s.p1_score + pos;
                        next.push(MultiPlayer {
                            p1_position: pos,
                            p1_score: sco,
                            p2_position: s.p2_position,
                            p2_score: s.p2_score,
                            worlds: s.worlds * count,
                        });
                    }
                }
                std::mem::swap(&mut next, &mut states);

                let (wins, not_wins) = states
                    .iter()
                    .partition::<Vec<MultiPlayer>, _>(|s| s.p1_score >= 21);

                p1_wins += wins.iter().map(|p| p.worlds).sum::<u64>();

                states = not_wins;

                println!("P2 moves, {} states", states.len());
                next.clear();
                for s in states.iter() {
                    for (change, count) in &counts {
                        let pos = advance(s.p2_position, *change);
                        let sco = s.p2_score + pos;
                        next.push(MultiPlayer {
                            p1_position: s.p1_position,
                            p1_score: s.p1_score,
                            p2_position: pos,
                            p2_score: sco,
                            worlds: s.worlds * count,
                        });
                    }
                }

                std::mem::swap(&mut next, &mut states);

                let (wins, not_wins) = states
                    .iter()
                    .partition::<Vec<MultiPlayer>, _>(|s| s.p2_score >= 21);

                p2_wins += wins.iter().map(|p| p.worlds).sum::<u64>();

                states = not_wins;
            }

            println!("Total P1: {p1_wins}");
            println!("Total P2: {p2_wins}");

            println!("Report: {}", p1_wins.max(p2_wins));
        }
    }
}

fn advance(pos: i32, add: i32) -> i32 {
    (pos - 1 + add) % 10 + 1
}
