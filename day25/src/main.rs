use std::fs;

fn main() {
    let filename = "assets/input-real.txt";

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let data: Vec<&str> = contents.lines().collect();

    let mut grid: Vec<Vec<char>> = data
        .iter()
        .map(|row| row.chars().collect::<Vec<char>>())
        .collect();

    let rows = grid.len();
    let cols = grid[0].len();

    let mut steps = 0;

    loop {
        let mut moved = false;
        let mut new_grid = grid.clone();

        for row in 0..rows {
            for col in 0..cols {
                if grid[row][col] != '>' {
                    continue;
                }

                let new_col = (col + 1) % cols;

                if grid[row][new_col] == '.' {
                    new_grid[row][col] = '.';
                    new_grid[row][new_col] = '>';
                    moved = true;
                }
            }
        }

        grid = new_grid.clone();

        for row in 0..rows {
            for col in 0..cols {
                if grid[row][col] != 'v' {
                    continue;
                }

                let new_row = (row + 1) % rows;

                if grid[new_row][col] == '.' {
                    new_grid[row][col] = '.';
                    new_grid[new_row][col] = 'v';
                    moved = true;
                }
            }
        }

        steps += 1;

        grid = new_grid;

        if !moved {
            break;
        }
    }
}
