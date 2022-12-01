use std::{collections::LinkedList, fs};

fn step(energy_levels: &mut Vec<Vec<usize>>) -> usize {
    let mut flash_stack: LinkedList<(usize, usize)> = LinkedList::new();
    let mut flashed: Vec<Vec<bool>> =
        vec![vec![false; energy_levels[0].len()]; energy_levels.len()];

    let mut num_flashes: usize = 0;

    let adjacency_offsets: [(i32, i32); 8] = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    for i in 0..energy_levels.len() {
        for j in 0..energy_levels[i].len() {
            energy_levels[i][j] += 1;
            energy_levels[i][j] %= 10;

            if energy_levels[i][j] == 0 {
                flash_stack.push_back((i, j));
                flashed[i][j] = true;
            }
        }
    }

    while !flash_stack.is_empty() {
        let (row, col) = flash_stack.pop_back().unwrap();

        adjacency_offsets.iter().for_each(|offset| {
            let row_offset_signed = (row as i32) + offset.0;
            let col_offset_signed = (col as i32) + offset.1;

            if row_offset_signed < 0 || col_offset_signed < 0 {
                return;
            }

            let row_offset = row_offset_signed as usize;
            let col_offset = col_offset_signed as usize;

            if row_offset >= energy_levels.len() || col_offset >= energy_levels[0].len() {
                return;
            }

            if !flashed[row_offset][col_offset] {
                energy_levels[row_offset][col_offset] += 1;
                energy_levels[row_offset][col_offset] %= 10;

                if energy_levels[row_offset][col_offset] == 0 {
                    flash_stack.push_back((row_offset, col_offset));
                    flashed[row_offset][col_offset] = true;
                }
            }
        });

        flashed[row][col] = true;
        num_flashes += 1;
    }

    return num_flashes;
}

fn main() {
    let filename = "assets/input-real.txt";

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let data: Vec<&str> = contents.lines().collect();

    let mut energy_levels: Vec<Vec<usize>> = data
        .into_iter()
        .map(|row| {
            row.chars()
                .map(|energy_level| energy_level.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();

    let mut num_flashes = 0;

    for _ in 0..100 {
        num_flashes += step(&mut energy_levels);
    }

    println!("{}", num_flashes);
}
