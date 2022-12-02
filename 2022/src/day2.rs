use std::{str::FromStr, error::Error, fmt};

#[derive(Debug)]
struct ParseChoiceError;

impl Error for ParseChoiceError {}

impl fmt::Display for ParseChoiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Choice should be A, B, C, X, Y, or Z")
    }
}

#[derive(PartialEq, Clone, Copy)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

#[derive(PartialEq, Clone, Copy)]
enum GameResult {
    Win,
    Lose,
    Tie,
}

impl FromStr for GameResult {
    type Err = ParseChoiceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().count() != 1 {
            return Err(ParseChoiceError);
        }

        return match s.chars().nth(0).unwrap() {
            'X' => Ok(GameResult::Lose),
            'Y' => Ok(GameResult::Tie),
            'Z' => Ok(GameResult::Win),
            _ => Err(ParseChoiceError),
        };
    }
}

impl Choice {
    fn win(self, other: Choice) -> bool {
        return (self == Choice::Rock && other == Choice::Scissors) ||
            (self == Choice::Paper && other == Choice::Rock) ||
            (self == Choice::Scissors && other == Choice::Paper);
    }

    pub fn cmp1(self, other: Choice) -> usize {
        let mut res = match self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        };

        res += if self.win(other) {
            6
        } else if self == other {
            3
        } else {
            0
        };

        return res;
    }

    pub fn cmp2(self, other: GameResult) -> usize {
        let your_choice = if other == GameResult::Win {
            match self {
                Choice::Rock => Choice::Paper,
                Choice::Paper => Choice::Scissors,
                Choice::Scissors => Choice::Rock,
            }
        } else if other == GameResult::Lose {
            match self {
                Choice::Rock => Choice::Scissors,
                Choice::Paper => Choice::Rock,
                Choice::Scissors => Choice::Paper,
            }
        } else {
            self
        };

        return match your_choice {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        } + match other {
            GameResult::Win => 6,
            GameResult::Tie => 3,
            GameResult::Lose => 0,
        };
    }
}

impl FromStr for Choice {
    type Err = ParseChoiceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().count() != 1 {
            return Err(ParseChoiceError);
        }

        return match s.chars().nth(0).unwrap() {
            'A' | 'X' => Ok(Choice::Rock),
            'B' | 'Y' => Ok(Choice::Paper),
            'C' | 'Z' => Ok(Choice::Scissors),
            _ => Err(ParseChoiceError),
        };
    }
}

fn parse_input1(input: String) -> Vec<(Choice, Choice)> {
    return input
        .lines()
        .map(|line| {
            let (play, response) = line.split_once(' ').unwrap();
            return (play.parse().unwrap(), response.parse().unwrap());
        })
        .collect();
}

fn part1(input: String) -> usize {
    return parse_input1(input)
        .iter()
        .fold(0, |acc, x| acc + x.1.cmp1(x.0));
}

fn parse_input2(input: String) -> Vec<(Choice, GameResult)> {
    return input
        .lines()
        .map(|line| {
            let (play, response) = line.split_once(' ').unwrap();
            return (play.parse().unwrap(), response.parse().unwrap());
        })
        .collect();
}

fn part2(input: String) -> usize {
    return parse_input2(input)
        .iter()
        .fold(0, |acc, x| acc + x.0.cmp2(x.1));
}

fn main() {
    let input = include_str!("./input/day2.input");
    println!("part1: {}", part1(input.to_string()));
    println!("part2: {}", part2(input.to_string()));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "\
A Y
B X
C Z
";
        assert_eq!(part1(input.to_string()), 15);
    }


    #[test]
    fn test_part2() {
        let input = "\
A Y
B X
C Z
";
        assert_eq!(part2(input.to_string()), 12);
    }
}
