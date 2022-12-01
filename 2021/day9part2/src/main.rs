use std::{collections::LinkedList, fs};

fn is_minima(height_levels: &Vec<Vec<u32>>, row: usize, col: usize) -> bool {
    let height = height_levels[row][col];
    return (row == 0 || height < height_levels[row - 1][col])
        && (row == height_levels.len() - 1 || height < height_levels[row + 1][col])
        && (col == 0 || height < height_levels[row][col - 1])
        && (col == height_levels[row].len() - 1 || height < height_levels[row][col + 1]);
}

fn find_basin_size(height_levels: &Vec<Vec<u32>>, minima: (usize, usize)) -> usize {
    let mut visited: Vec<Vec<bool>> =
        vec![vec![false; height_levels[0].len()]; height_levels.len()];
    let mut queue: LinkedList<(usize, usize)> = LinkedList::new();
    let mut basin_size = 1;
    let mut row = minima.0;
    let mut col = minima.1;

    visited[row][col] = true;
    queue.push_back(minima);

    while !queue.is_empty() {
        let s = queue.pop_front();
        row = s.unwrap().0;
        col = s.unwrap().1;

        if row != 0 && height_levels[row - 1][col] != 9 && !visited[row - 1][col] {
            queue.push_back((row - 1, col));
            visited[row - 1][col] = true;
            basin_size += 1;
        }

        if row != height_levels.len() - 1
            && height_levels[row + 1][col] != 9
            && !visited[row + 1][col]
        {
            queue.push_back((row + 1, col));
            visited[row + 1][col] = true;
            basin_size += 1;
        }

        if col != 0 && height_levels[row][col - 1] != 9 && !visited[row][col - 1] {
            queue.push_back((row, col - 1));
            visited[row][col - 1] = true;
            basin_size += 1;
        }

        if col != height_levels[0].len() - 1
            && height_levels[row][col + 1] != 9
            && !visited[row][col + 1]
        {
            queue.push_back((row, col + 1));
            visited[row][col + 1] = true;
            basin_size += 1;
        }
    }

    return basin_size;
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

    let mut minima: LinkedList<(usize, usize)> = LinkedList::new();
    let mut largest_three_basin_sizes: [u32; 3] = [0, 0, 0];

    for (i, row) in height_levels.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            if is_minima(&height_levels, i, j) {
                minima.push_back((i, j));
            }
        }
    }

    for min in minima.iter() {
        let basin_size = find_basin_size(&height_levels, *min);
        let mut min_idx = 0;

        for (i, size) in largest_three_basin_sizes.iter().enumerate() {
            if size < &largest_three_basin_sizes[min_idx] {
                min_idx = i;
            }
        }

        if basin_size > largest_three_basin_sizes[min_idx] as usize {
            largest_three_basin_sizes[min_idx] = basin_size as u32;
        }
    }

    println!(
        "{:?}",
        largest_three_basin_sizes[0] * largest_three_basin_sizes[1] * largest_three_basin_sizes[2]
    );
}
