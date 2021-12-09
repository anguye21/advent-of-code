use std::fs;

fn is_minima(height_levels: &Vec<Vec<u32>>, row: usize, col: usize) -> bool {
    let height = height_levels[row][col];
    return (row == 0 || height < height_levels[row - 1][col])
        && (row == height_levels.len() - 1 || height < height_levels[row + 1][col])
        && (col == 0 || height < height_levels[row][col - 1])
        && (col == height_levels[row].len() - 1 || height < height_levels[row][col + 1]);
}

fn main() {
    let filename = "assets/input-real.txt";

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let data: Vec<&str> = contents.lines().collect();

    let height_levels: Vec<Vec<u32>> = data
        .into_iter()
        .map(|row| {
            row.chars()
                .map(|height| height.to_digit(10).unwrap())
                .collect()
        })
        .collect();

    let mut sum = 0;

    for (i, row) in height_levels.iter().enumerate() {
        for (j, height) in row.iter().enumerate() {
            if is_minima(&height_levels, i, j) {
r               sum += height + 1;
            }
        }
    }

    println!("{}", sum);
}
