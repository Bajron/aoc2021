use aoc2021::input::read_lines;

fn main() {
    let input = read_lines().ok().and_then(|lines| {
        let mut it = lines.filter_map(|l| l.ok());

        let points = it
            .by_ref()
            .map_while(|l| {
                if l.is_empty() {
                    None
                } else {
                    Some(
                        l.split(",")
                            .filter_map(|x| x.parse::<usize>().ok())
                            .collect::<Vec<usize>>(),
                    )
                }
            })
            .filter_map(|x| match x[..] {
                [x, y] => Some((x, y)),
                _ => None,
            })
            .collect::<Vec<(usize, usize)>>();

        let folds = it
            .filter_map(
                |x| match &x.split("=").map(|x| x.to_owned()).collect::<Vec<String>>()[..] {
                    [instr, value] => match value.parse::<usize>().ok() {
                        Some(v) => Some((instr.clone(), v)),
                        _ => None,
                    },
                    _ => None,
                },
            )
            .collect::<Vec<(String, usize)>>();
        Some((points, folds))
    });

    if let Some((points, folds)) = input {
        let initial_board = make_board(&points);
        show_board(&initial_board);
        println!("Markings: {}", count_board(&initial_board));
        println!("{points:?}");
        println!("{folds:?}");

        let mut folded = points.clone();
        for fold in folds {
            let value = fold.1;
            let changer: Box<dyn Fn(&(usize, usize)) -> (usize, usize)> = if fold.0.ends_with("x") {
                Box::new(|&(x, y): &(usize, usize)| {
                    (if x > value { value - (x - value) } else { x }, y)
                })
            } else {
                Box::new(|&(x, y): &(usize, usize)| {
                    (x, if y > value { value - (y - value) } else { y })
                })
            };

            folded = folded.iter().map(changer).collect::<Vec<(usize, usize)>>();

            let board = make_board(&folded);
            show_board(&board);
            println!("Markings: {}", count_board(&board));
            println!();
            // break;
        }
    }
}

fn make_board(points: &Vec<(usize, usize)>) -> Vec<String> {
    let (mx, my) = points
        .iter()
        .fold((0, 0), |acc, &(x, y)| (acc.0.max(x), acc.1.max(y)));

    let mut board: Vec<String> = vec![".".repeat(mx + 1); my + 1];

    points.iter().for_each(|&(x, y)| {
        board[y].replace_range(x..x + 1, "#");
    });

    board
}

fn count_board(board: &Vec<String>) -> usize {
    board
        .iter()
        .flat_map(|x| x.chars())
        .filter(|&c| c == '#')
        .count()
}

fn show_board(board: &Vec<String>) {
    for l in board {
        println!("{l}");
    }
}
