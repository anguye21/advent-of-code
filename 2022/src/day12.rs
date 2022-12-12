use std::collections::{VecDeque, HashSet};

fn parse_input(input: &str) -> (Vec<Vec<usize>>, (usize, usize), (usize, usize)) {
    let height_map = input
        .lines()
        .map(|row|
             row.chars().map(|elevation| if elevation == 'S' {
                 0
             } else if elevation == 'E' {
                 25
             } else {
                 elevation as usize - 'a' as usize
             })
             .collect::<Vec<usize>>()
        )
        .collect::<Vec<Vec<usize>>>();

    let start_pos = input
        .lines()
        .flat_map(|line| line.chars())
        .position(|x| x == 'S')
        .unwrap();

    let start_pos = (start_pos / height_map[0].len(), start_pos % height_map[0].len());

    let end_pos = input
        .lines()
        .flat_map(|line| line.chars())
        .position(|x| x == 'E')
        .unwrap();

    let end_pos = (end_pos / height_map[0].len(), end_pos % height_map[0].len());

    return (height_map, start_pos, end_pos);
}

fn bfs_point(grid: &Vec<Vec<usize>>, start: (usize, usize), end: (usize, usize)) -> usize {
    let mut queue = VecDeque::from([(start.0, start.1, 0)]);
    let mut visited = HashSet::from([start]);

    while queue.len() > 0 {
        let (i, j, steps) = queue.pop_back().unwrap();

        if (i, j) == end {
            return steps;
        }

        if i > 0 && (grid[i-1][j] < grid[i][j] || grid[i-1][j] - grid[i][j] <= 1) && !visited.contains(&(i-1, j)){
            queue.push_front((i-1, j, steps+1));
            visited.insert((i-1, j));
        }

        if j > 0 && (grid[i][j-1] < grid[i][j] || grid[i][j-1] - grid[i][j] <= 1) && !visited.contains(&(i, j-1)){
            queue.push_front((i, j-1, steps+1));
            visited.insert((i, j-1));
        }

        if i < grid.len() - 1 && (grid[i+1][j] < grid[i][j] || grid[i+1][j] - grid[i][j] <= 1) && !visited.contains(&(i+1, j)){
            queue.push_front((i+1, j, steps+1));
            visited.insert((i+1, j));
        }

        if j < grid[0].len() - 1 && (grid[i][j+1] < grid[i][j] || grid[i][j+1] - grid[i][j] <= 1) && !visited.contains(&(i, j+1)){
            queue.push_front((i, j+1, steps+1));
            visited.insert((i, j+1));
        }
    }

    unreachable!();
}

fn part1(input: &str) -> usize {
    let (height_map, start_pos, end_pos) = parse_input(input);
    return bfs_point(&height_map, start_pos, end_pos);
}

fn bfs_value(grid: &Vec<Vec<usize>>, start: (usize, usize), value: usize) -> usize {
    let mut queue = VecDeque::from([(start.0, start.1, 0)]);
    let mut visited = HashSet::from([start]);

    while queue.len() > 0 {
        let (i, j, steps) = queue.pop_back().unwrap();

        if grid[i][j] == value {
            return steps;
        }

        if i > 0 && (grid[i-1][j] > grid[i][j] || grid[i][j] - grid[i-1][j] <= 1) && !visited.contains(&(i-1, j)){
            queue.push_front((i-1, j, steps+1));
            visited.insert((i-1, j));
        }

        if j > 0 && (grid[i][j-1] > grid[i][j] || grid[i][j] - grid[i][j-1]  <= 1) && !visited.contains(&(i, j-1)){
            queue.push_front((i, j-1, steps+1));
            visited.insert((i, j-1));
        }

        if i < grid.len() - 1 && (grid[i+1][j] > grid[i][j] || grid[i][j] - grid[i+1][j]  <= 1) && !visited.contains(&(i+1, j)){
            queue.push_front((i+1, j, steps+1));
            visited.insert((i+1, j));
        }

        if j < grid[0].len() - 1 && (grid[i][j+1] > grid[i][j] || grid[i][j] - grid[i][j+1]  <= 1) && !visited.contains(&(i, j+1)){
            queue.push_front((i, j+1, steps+1));
            visited.insert((i, j+1));
        }
    }

    unreachable!();
}

fn part2(input: &str) -> usize {
    let (height_map, _, end_pos) = parse_input(input);
    return bfs_value(&height_map, end_pos, 0);
}

fn main() {
    let input = include_str!("./input/day12.input");
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "\
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

        assert_eq!(part1(input), 31);
    }


    #[test]
    fn test_part2() {
        let input = "\
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

        assert_eq!(part2(input), 29);
    }
}
