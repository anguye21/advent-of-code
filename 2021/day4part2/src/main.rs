use std::fs;

fn board_won(marked_board: &Vec<Vec<i32>>) -> bool {
    for row in marked_board {
        if row.iter().sum::<i32>() == 5 {
            return true;
        }
    }

    for col in 0..5 {
        let mut sum: i32 = 0;

        for row in 0..5 {
            sum += marked_board[row][col];
        }

        if sum == 5 {
            return true;
        }
    }

    return false;
}

fn main() {
    let filename = "assets/input-real.txt";

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let data: Vec<&str> = contents.lines().collect();

    let draws: Vec<&str> = data[0].split(",").collect();

    let mut boards: Vec<Vec<Vec<&str>>> = Vec::new();
    let mut board: Vec<Vec<&str>> = vec![vec![""; 5]; 5];
    let mut row: usize = 0;
    let mut col: usize = 0;

    let mut iter = data.iter();

    iter.next();
    iter.next();

    for line in iter {
        if row == 5 {
            boards.push(board);
            row = 0;
            col = 0;
            board = vec![vec![""; 5]; 5];
            continue;
        }

        for num in line.split(" ") {
            if num == "" {
                continue;
            }

            board[row][col] = num;
            col += 1;
        }

        col = 0;
        row += 1;
    }

    boards.push(board);

    let mut most_moves = -1;
    let mut worst_board: i32 = -1;
    let mut last_called = -1;
    let mut worst_board_marked: Vec<Vec<i32>> = vec![vec![0; 5]; 5];
    let mut board_idx = 0;

    for board in &boards {
        let mut board_marked: Vec<Vec<i32>> = vec![vec![0; 5]; 5];
        let mut num_moves = 0;
        for draw in &draws {
            for row in 0..5 {
                for col in 0..5 {
                    if &board[row][col] == draw {
                        board_marked[row][col] = 1;
                    }
                }
            }

            num_moves += 1;

            if board_won(&board_marked) {
                if worst_board == -1 || num_moves > most_moves {
                    worst_board = board_idx;
                    most_moves = num_moves;
                    worst_board_marked = board_marked;
                    last_called = (*draw).parse::<i32>().unwrap();
                }
                break;
            }
        }
        board_idx += 1;
    }

    let mut sum: i32 = 0;

    for row in 0..5 {
        for col in 0..5 {
            if worst_board_marked[row][col] == 0 {
                sum += boards[worst_board as usize][row][col]
                    .parse::<i32>()
                    .unwrap();
            }
        }
    }

    println!("{}", sum * last_called);
}
