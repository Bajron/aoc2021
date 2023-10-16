use aoc2021::input::*;
use std::collections::HashMap;

enum Result {
    Corrupted(char),
    Unexpected(),
    Incomplete(Vec<char>),
    Ok(),
}

fn main() {
    let input = read_lines().ok().and_then(|lines| {
        let code_lines = lines.filter_map(|l| l.ok()).collect::<Vec<String>>();
        Some(code_lines)
    });

    if let Some(code_lines) = input {
        let corrupted_score_mapping: HashMap<char, i32> =
            vec![(')', 3), (']', 57), ('}', 1197), ('>', 25137)]
                .iter()
                .copied()
                .collect();

        let results: Vec<Result> = code_lines.iter().map(process).collect();
        let corrupted_score: i32 = results
            .iter()
            .filter_map(|x| match x {
                Result::Corrupted(c) => Some(c),
                _ => None,
            })
            .map(|c| corrupted_score_mapping[&c])
            .sum();

        println!("Corrupted score: {corrupted_score}");

        let complete_score_mapping: HashMap<char, i32> =
            vec![('(', 1), ('[', 2), ('{', 3), ('<', 4)]
                .iter()
                .copied()
                .collect();
        let mut autocomplete_scores = results
            .iter()
            .filter_map(|x| match x {
                Result::Incomplete(c) => Some(c),
                _ => None,
            })
            .map(|c| {
                c.iter()
                    .map(|x| complete_score_mapping[x] as i64)
                    .fold(0, |acc, v| 5 * acc + v)
            })
            .collect::<Vec<i64>>();
        autocomplete_scores.sort();
        let n = autocomplete_scores.len();
        let middle_ac_score = autocomplete_scores[n / 2];
        println!("Autocomplete score: {middle_ac_score}");
    }
}

fn process(line: &String) -> Result {
    let mut stack: Vec<char> = vec![];
    for c in line.chars() {
        if "([{<".contains(c) {
            stack.push(c)
        } else {
            match stack.last() {
                Some(&last) => {
                    if last == '(' && c == ')'
                        || last == '<' && c == '>'
                        || last == '[' && c == ']'
                        || last == '{' && c == '}'
                    {
                        stack.pop();
                    } else {
                        return Result::Corrupted(c);
                    }
                }
                None => return Result::Unexpected(),
            }
        }
    }
    if stack.is_empty() {
        Result::Ok()
    } else {
        stack.reverse();
        Result::Incomplete(stack)
    }
}
