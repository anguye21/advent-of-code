use std::{collections::HashSet, str::FromStr};

use aoc2022::ParseInputError;

#[derive(Debug)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

impl FromStr for Direction {
    type Err = ParseInputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return match s.chars().nth(0) {
            Some('R') => Ok(Direction::Right),
            Some('L') => Ok(Direction::Left),
            Some('U') => Ok(Direction::Up),
            Some('D') => Ok(Direction::Down),
            _ => Err(ParseInputError),
        };
    }
}

#[derive(Debug)]
struct Movement {
    direction: Direction,
    amount: isize,
}

impl FromStr for Movement {
    type Err = ParseInputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, amount) = s.split_once(" ").ok_or(ParseInputError)?;
        let direction = dir.parse()?;
        let amount = amount.parse().map_err(|_| ParseInputError)?;

        return Ok(Movement { direction, amount });
    }
}

fn part1(input: &str) -> usize {
    let mut tail_visited: HashSet<(isize, isize)> = HashSet::from([(0, 0)]);

    let mut tail_pos: (isize, isize) = (0, 0);
    let mut head_pos: (isize, isize) = (0, 0);

    for line in input.lines() {
        let mut movement = line.parse::<Movement>().unwrap();

        while movement.amount > 0 {
            match movement.direction {
                Direction::Up => head_pos.1 += 1,
                Direction::Down => head_pos.1 -= 1,
                Direction::Left => head_pos.0 -= 1,
                Direction::Right => head_pos.0 += 1,
            };

            let diff = (head_pos.0 - tail_pos.0, head_pos.1 - tail_pos.1);

            if diff.0 != 0 && diff.1 != 0 && (diff.0.abs() > 1 || diff.1.abs() > 1) {
                if head_pos.0 > tail_pos.0 {
                    tail_pos.0 += 1;
                } else {
                    tail_pos.0 -= 1;
                }

                if head_pos.1 > tail_pos.1 {
                    tail_pos.1 += 1;
                } else {
                    tail_pos.1 -= 1;
                }
            } else if diff.0.abs() > 1 {
                if head_pos.0 > tail_pos.0 {
                    tail_pos.0 += 1;
                } else {
                    tail_pos.0 -= 1;
                }
            } else if diff.1.abs() > 1 {
                if head_pos.1 > tail_pos.1 {
                    tail_pos.1 += 1;
                } else {
                    tail_pos.1 -= 1;
                }
            }

            tail_visited.insert(tail_pos);
            movement.amount -= 1;
        }
    }

    return tail_visited.len();
}

fn part2(input: &str) -> usize {
    let mut tail_visited: HashSet<(isize, isize)> = HashSet::from([(0, 0)]);

    let mut positions: [(isize, isize); 10] = [(0, 0); 10];

    for line in input.lines() {
        let mut movement = line.parse::<Movement>().unwrap();

        while movement.amount > 0 {
            match movement.direction {
                Direction::Up => positions[0].1 += 1,
                Direction::Down => positions[0].1 -= 1,
                Direction::Left => positions[0].0 -= 1,
                Direction::Right => positions[0].0 += 1,
            };

            for i in 0..9 {
                let head = positions[i];
                let mut tail = positions[i+1];

                let diff = (head.0 - tail.0, head.1 - tail.1);

                if diff.0 != 0 && diff.1 != 0 && (diff.0.abs() > 1 || diff.1.abs() > 1) {
                    if head.0 > tail.0 {
                        tail.0 += 1;
                    } else {
                        tail.0 -= 1;
                    }

                    if head.1 > tail.1 {
                        tail.1 += 1;
                    } else {
                        tail.1 -= 1;
                    }
                } else if diff.0.abs() > 1 {
                    if head.0 > tail.0 {
                        tail.0 += 1;
                    } else {
                        tail.0 -= 1;
                    }
                } else if diff.1.abs() > 1 {
                    if head.1 > tail.1 {
                        tail.1 += 1;
                    } else {
                        tail.1 -= 1;
                    }
                }

                positions[i+1] = tail;
            }

            tail_visited.insert(positions[9]);
            movement.amount -= 1;
        }
    }

    return tail_visited.len();
}

fn main() {
    let input = include_str!("./input/day9.input");
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "\
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

        assert_eq!(part1(input), 13);
    }

    #[test]
    fn test_part2() {
        let input = "\
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

        assert_eq!(part2(input), 36);
    }
}
