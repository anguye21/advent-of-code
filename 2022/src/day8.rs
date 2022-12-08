use std::cmp;

fn parse_input(input: &str) -> Vec<Vec<usize>> {
    return input
        .lines()
        .map(|line| {
            line.chars().map(|c| c.to_digit(10).unwrap() as usize).collect()
        })
        .collect();
}

fn is_visiable(i: usize, j: usize, grid: &Vec<Vec<usize>>) -> bool {
    if i == 0 || i == grid.len() - 1 || j == 0 || j == grid[0].len() {
        return true;
    }

    let mut visible = true;

    // up
    for k in 0..i {
        if grid[k][j] >= grid[i][j] {
            visible = false;
            break
        }
    }

    if visible {
        return true
    }

    // down
    visible = true;
    for k in i+1..grid.len() {
        if grid[k][j] >= grid[i][j] {
            visible = false;
            break;
        }
    }

    if visible {
        return true;
    }

    // left
    visible = true;
    for k in 0..j {
        if grid[i][k] >= grid[i][j] {
            visible = false;
            break;
        }
    }

    if visible {
        return true;
    }

    // right
    visible = true;
    for k in j+1..grid[0].len() {
        if grid[i][k] >= grid[i][j] {
            visible = false;
            break;
        }
    }

    return visible;
}

fn part1(input: &str) -> usize {
    let grid = parse_input(input);
    let mut res = 0;

    for (i, row) in grid.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            res += is_visiable(i, j, &grid) as usize;
        }
    }

    return res;
}

fn scenic_score(i: usize, j: usize, grid: &Vec<Vec<usize>>) -> usize {
    let mut up = 0;
    let mut down = 0;
    let mut left = 0;
    let mut right = 0;

    // up
    for k in (0..i).rev() {
        up += 1;
        if grid[k][j] >= grid[i][j] {
            break;
        }
    }

    // down
    for k in i+1..grid.len() {
        down += 1;
        if grid[k][j] >= grid[i][j] {
            break;
        }
    }

    // left
    for k in (0..j).rev() {
        left += 1;
        if grid[i][k] >= grid[i][j] {
            break
        }
    }

    // right
    for k in j+1..grid[0].len() {
        right += 1;
        if grid[i][k] >= grid[i][j] {
            break;
        }
    }

    return left*right*up*down;
}

fn part2(input: &str) -> usize {
    let grid = parse_input(input);
    let mut res = 0;

    for (i, row) in grid.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            res = cmp::max(res, scenic_score(i, j, &grid));
        }
    }

    return res;
}

fn main() {
    let input = include_str!("input/day8.input");
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let calories = "\
30373
25512
65332
33549
35390";

        assert_eq!(part1(calories), 21);
    }

    #[test]
    fn test_part2() {
        let calories = "\
30373
25512
65332
33549
35390";

        assert_eq!(part2(calories), 8);
    }
}
