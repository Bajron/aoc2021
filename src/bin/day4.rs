use aoc2021::input::read_lines;

type Board = Vec<Vec<i32>>;

struct BoardMarks {
    marks: Vec<Vec<bool>>,
    in_row: Vec<i32>,
    in_col: Vec<i32>,
}

impl BoardMarks {
    fn new(target: &Board) -> BoardMarks {
        BoardMarks {
            marks: vec![vec![false; target[0].len()]; target.len()],
            in_row: vec![0; target.len()],
            in_col: vec![0; target[0].len()],
        }
    }

    fn mark(&mut self, target: &Board, number: i32) {
        for (ri, row) in target.iter().enumerate() {
            for (ci, &value) in row.iter().enumerate() {
                if value == number && !self.marks[ri][ci] {
                    self.marks[ri][ci] = true;
                    self.in_row[ri] += 1;
                    self.in_col[ci] += 1;
                }
            }
        }
    }

    fn is_bingo(&self) -> bool {
        return self
            .in_col
            .iter()
            .chain(self.in_row.iter())
            .any(|&x| x == 5);
    }

    fn score(&self, target: &Board) -> i32 {
        self.marks
            .iter()
            .flatten()
            .zip(target.iter().flatten())
            .map(|(&m, &v)| if !m { v } else { 0 })
            .sum()
    }
}

fn main() {
    let input = read_lines().ok().and_then(|lines| {
        let mut it = lines.filter_map(|l| l.ok());

        let numbers: Vec<i32> = if let Some(first) = it.next() {
            first
                .split(",")
                .filter_map(|s| s.trim().parse::<i32>().ok())
                .collect()
        } else {
            vec![]
        };

        let mut boards: Vec<Board> = vec![];
        loop {
            let board = it
                .by_ref()
                .skip_while(|x| x.trim().is_empty())
                .map_while(|x| {
                    let row = x
                        .split_whitespace()
                        .filter_map(|s| s.parse::<i32>().ok())
                        .collect::<Vec<i32>>();
                    if row.is_empty() {
                        None
                    } else {
                        Some(row)
                    }
                })
                .collect::<Vec<Vec<i32>>>();

            if board.len() < 5 {
                if board.len() > 0 {
                    eprintln!(" ** Incomplete board on input! {:?}", board);
                }
                break;
            }
            boards.push(board);
        }
        Some((numbers, boards))
    });

    if let Some((numbers, boards)) = input {
        println!("{:?}", numbers);
        println!("{:?}", boards);

        let mut marks: Vec<BoardMarks> = boards.iter().map(|b| BoardMarks::new(b)).collect();

        for number in numbers {
            for (i, (board, m)) in boards.iter().zip(marks.iter_mut()).enumerate() {
                if m.is_bingo() {
                    continue;
                }
                m.mark(board, number);
                if m.is_bingo() {
                    println!("Bingo! at number {number}");
                    println!("Winning board is {i}");
                    let board_sum = m.score(board);
                    println!("Board sum {board_sum}");
                    println!("Score is {}", board_sum * number);
                    println!();
                }
            }
        }
    }
}
