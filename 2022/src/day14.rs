use std::{collections::HashMap, cmp};

#[derive(Debug)]
enum Space {
    Rock,
    Sand,
    Air
}

fn parse_input(input: &str) -> HashMap<(usize, usize), Space> {
    let mut slice = HashMap::new();

    for line in input.lines() {
        let verticies = line.split(" -> ");
        let mut last_x = None;
        let mut last_y = None;

        for vertex in verticies {
            let (x, y) = vertex.split_once(",").unwrap();
            let x = x.parse::<usize>().unwrap();
            let y = y.parse::<usize>().unwrap();

            if (last_x, last_y) == (None, None) {
                slice.insert((x, y), Space::Rock);
            } else if last_x != Some(x) {
                for i in cmp::min(last_x.unwrap(), x)..=cmp::max(last_x.unwrap(), x) {
                    slice.insert((i, y), Space::Rock);
                }
            } else if last_y != Some(y) {
                for i in cmp::min(last_y.unwrap(), y)..=cmp::max(last_y.unwrap(), y) {
                    slice.insert((x, i), Space::Rock);
                }
            } else {
                unreachable!();
            }

            last_x = Some(x);
            last_y = Some(y);
        }
    }

    return slice;
}

fn part1(input: &str) -> usize {
    let mut count = 0;
    let mut slice = parse_input(input);
    let max_rock_y = *slice
        .iter()
        .map(|((_, y), _)| y)
        .max()
        .unwrap();

    'outer: loop {
        let mut x = 500;
        let mut y = 0;

        'inner: loop {
            if !slice.contains_key(&(x, y+1)) {
                y += 1;
            } else if !slice.contains_key(&(x-1, y+1)) {
                x -= 1;
                y += 1;
            } else if !slice.contains_key(&(x+1, y+1)) {
                x += 1;
                y += 1;
            } else {
                slice.insert((x, y), Space::Sand);
                count += 1;
                break 'inner;
            }

            if y > max_rock_y {
                break 'outer;
            }
        }
    }

    return count;
}

fn part2(input: &str) -> usize {
    let mut count = 0;
    let mut slice = parse_input(input);
    let max_rock_y = *slice
        .iter()
        .map(|((_, y), _)| y)
        .max()
        .unwrap();
    let floor = max_rock_y + 2;

    while !slice.contains_key(&(500, 0)) {
        let mut x = 500;
        let mut y = 0;

        loop {
            if y == floor - 1 {
                slice.insert((x, y), Space::Sand);
                count += 1;
                break;
            } else if !slice.contains_key(&(x, y+1)) {
                y += 1;
            } else if !slice.contains_key(&(x-1, y+1)) {
                x -= 1;
                y += 1;
            } else if !slice.contains_key(&(x+1, y+1)) {
                x += 1;
                y += 1;
            } else {
                slice.insert((x, y), Space::Sand);
                count += 1;
                break;
            }
        }
    }

    return count;
}

fn main() {
    let input = include_str!("./input/day14.input");
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "\
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

        assert_eq!(part1(input), 24);
    }

    #[test]
    fn test_part2() {
        let input = "\
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

        assert_eq!(part2(input), 93);
    }
}
