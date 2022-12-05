use std::{collections::VecDeque, str::FromStr, error::Error, fmt};


#[derive(Debug)]
struct ParseMoveError;

impl Error for ParseMoveError {}

impl fmt::Display for ParseMoveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "cannot parse move")
    }
}
#[derive(Debug)]
struct Move {
    amount: usize,
    from: usize,
    to: usize,
}

impl FromStr for Move {
    type Err = ParseMoveError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = &s[5..];


        let amount_end = s.chars().position(|c| c == ' ').unwrap();
        let amount = s[..amount_end].parse().map_err(|_| ParseMoveError)?;

        let s = &s[amount_end+6..];

        let (from, to) = s.split_once(" to ").unwrap();
        return Ok(Move {
            amount,
            from: from.parse().map_err(|_| ParseMoveError)?,
            to: to.parse().map_err(|_| ParseMoveError)?,
        });
    }
}

fn parse_input(input: &str) -> (Vec<VecDeque<char>>, Vec<Move>)  {
    let (stacks_input, moves_input) = input.split_once("\n\n").unwrap();

    let mut stack_iter = stacks_input
        .lines()
        .rev();

    let num_stacks = stack_iter
        .next()
        .unwrap()
        .trim()
        .chars()
        .last()
        .unwrap() as usize - '0' as usize;

    let mut stacks: Vec<VecDeque<char>> = Vec::new();

    for _ in 0..num_stacks {
        stacks.push(VecDeque::new());
    }

    for line in stack_iter {
        for i in 0..num_stacks {
            let idx = i * 4 + 1;

            if let Some(c) = line.chars().nth(idx) {
                if c != ' ' {
                    stacks[i].push_back(c);
                }
            }

        }
    }

    let moves = moves_input
        .lines()
        .map(|line| line.parse::<Move>().unwrap())
        .collect();

    return (stacks, moves);
}

fn part1(input: &str) -> String {
    let (mut stacks, moves) = parse_input(input);
    let mut res = String::from("");

    for m in moves {
        for _ in 0..m.amount {
            if let Some(poped_val) = stacks[m.from - 1].pop_back() {
                stacks[m.to - 1].push_back(poped_val);
            }
        }
    }

    for stack in stacks {
        if let Some(back) = stack.back() {
            res.push(*back);
        }
    }

    return res;
}

fn part2(input: &str) -> String {
    let (mut stacks, moves) = parse_input(input);
    let mut res = String::from("");

    for m in moves {
        let mut tmp = Vec::new();
        for _ in 0..m.amount {
            if let Some(poped_val) = stacks[m.from - 1].pop_back() {
                tmp.push(poped_val);
            }
        }

        for c in tmp.iter().rev() {
            stacks[m.to - 1].push_back(*c);
        }

    }

    for stack in stacks {
        if let Some(back) = stack.back() {
            res.push(*back);
        }
    }

    return res;
}

fn main() {
    let input = include_str!("./input/day5.input");
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

        assert_eq!(part1(input), "CMZ".to_string());
    }

    #[test]
    fn test_part2() {
        let input = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

        assert_eq!(part2(input), "MCD".to_string());
    }
}
