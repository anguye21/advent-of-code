use std::{collections::HashSet, fs};

fn fold_paper_y(grid: &mut HashSet<(u32, u32)>, fold_instruction: (char, u32)) {
    let mut remove: Vec<(u32, u32)> = Vec::new();
    let mut add: Vec<(u32, u32)> = Vec::new();

    for point in grid.clone() {
        if point.1 > fold_instruction.1 {
            remove.push(point);
            add.push((point.0, fold_instruction.1 - (point.1 - fold_instruction.1)));
        }
    }

    for point in remove {
        grid.remove(&point);
    }

    for point in add {
        grid.insert(point);
    }
}

fn fold_paper_x(grid: &mut HashSet<(u32, u32)>, fold_instruction: (char, u32)) {
    let mut remove: Vec<(u32, u32)> = Vec::new();
    let mut add: Vec<(u32, u32)> = Vec::new();

    for point in grid.clone() {
        if point.0 > fold_instruction.1 {
            remove.push(point);
            add.push((fold_instruction.1 - (point.0 - fold_instruction.1), point.1));
        }
    }

    for point in remove {
        grid.remove(&point);
    }

    for point in add {
        grid.insert(point);
    }
}

fn main() {
    let filename = "assets/input-real.txt";

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let data: Vec<&str> = contents.lines().collect();
    let mut dots: HashSet<(u32, u32)> = HashSet::new();
    let mut folds: Vec<(char, u32)> = Vec::new();
    let mut iter = data.iter();
    let mut max_x = 0;
    let mut max_y = 0;

    loop {
        let point = iter.next().unwrap();

        if *point == "" {
            break;
        }

        let mut iter = point.splitn(2, ",");
        let x = iter.next().unwrap().parse::<u32>().unwrap();
        let y = iter.next().unwrap().parse::<u32>().unwrap();

        dots.insert((x, y));
    }

    for fold in iter {
        let mut iter = fold.splitn(2, "=");

        folds.push((
            iter.next().unwrap().chars().last().unwrap(),
            iter.next().unwrap().parse::<u32>().unwrap(),
        ));
    }

    for fold in folds {
        if fold.0 == 'x' {
            fold_paper_x(&mut dots, fold);
        } else {
            fold_paper_y(&mut dots, fold);
        }
    }

    for point in &dots {
        let x = point.0;
        let y = point.1;

        if x > max_x {
            max_x = x;
        }

        if y > max_y {
            max_y = y;
        }
    }

    let mut grid = vec![vec!['.'; (max_x + 1) as usize]; (max_y + 1) as usize];

    for point in &dots {
        grid[point.1 as usize][point.0 as usize] = '#';
    }

    for row in &grid {
        for point in row {
            print!("{}", point);
        }
        println!();
    }
}
