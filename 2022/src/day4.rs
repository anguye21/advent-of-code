use std::{str::FromStr, fmt, error::Error};

struct Range {
    low: usize,
    high: usize
}

#[derive(Debug)]
struct ParseRangeError;

impl Error for ParseRangeError {}

impl fmt::Display for ParseRangeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Choice should be A, B, C, X, Y, or Z")
    }
}

impl FromStr for Range {
    type Err = ParseRangeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (low, high) = s.split_once("-").ok_or(ParseRangeError)?;

        return Ok(Range {
            low: low.parse().map_err(|_| ParseRangeError)?,
            high: high.parse().map_err(|_| ParseRangeError)?,
        });
    }
}

fn part1(input: &str) -> usize {
    return input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(",").unwrap();

            return (left.parse::<Range>().unwrap(), right.parse::<Range>().unwrap());
        })
        .fold(0, |acc, x| {
            let (left, right) = x;

            return acc + if (left.low <= right.low && left.high >= right.high) ||
                       (left.low >= right.low && left.high <= right.high) {
                1
            } else {
                0
            };
        });
}

fn part2(input: &str) -> usize {
    return input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(",").unwrap();

            return (left.parse::<Range>().unwrap(), right.parse::<Range>().unwrap());
        })
        .fold(0, |acc, x| {
            let (left, right) = x;

            let does_overlap = (left.low <= right.low && left.high >= right.low) ||
                               (right.low <= left.low && right.high >= left.low);

            return acc + (does_overlap as usize);
        });
}

fn main() {
    let input = include_str!("./input/day4.input");
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

        assert_eq!(part1(input), 2);
    }

    #[test]
    fn test_part2() {
        let input = "\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

        assert_eq!(part2(input), 4);
    }
}
