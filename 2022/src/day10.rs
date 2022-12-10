use std::str::FromStr;

use aoc2022::ParseInputError;

#[derive(Debug)]
enum Instruction {
    AddX(isize),
    Noop,
}

impl FromStr for Instruction {
    type Err = ParseInputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let op = s.split(" ").collect::<Vec<&str>>();

        return match op[0] {
            "addx" => Ok(Instruction::AddX(op[1].parse().map_err(|_| ParseInputError)?)),
            "noop" => Ok(Instruction::Noop),
            _ => Err(ParseInputError),
        }
    }
}

struct Program {
    pub x: isize,
    pub pc: usize,
    pub instructions: Vec<Instruction>,
}

impl Program {
    pub fn new(program: &str) -> Program {
        let x = 1;
        let pc = 1;
        let mut instructions = Vec::new();

        for line in program.lines() {
            let instruction = line.parse::<Instruction>().unwrap();

            if let Instruction::AddX(_) = instruction {
                instructions.push(Instruction::Noop);
            }

            instructions.push(instruction);
        }

        return Program {
            x,
            pc,
            instructions,
        }
    }

    pub fn exec_next(&mut self) {
        if let Instruction::AddX(n) = self.instructions[self.pc - 1] {
            self.x += n;
        }

        self.pc += 1;
    }
}

fn part1(input: &str) -> isize {
    let mut program = Program::new(input);
    let mut sum = 0;

    for _ in 0..program.instructions.len() {
        if program.pc >= 20 && (program.pc - 20) % 40 == 0 {
            sum += program.x * program.pc as isize;
        }

        program.exec_next();
    }

    return sum;
}

fn part2(input: &str) -> String {
    let mut program = Program::new(input);
    let mut crt = [['.'; 40]; 6];

    for _ in 0..program.instructions.len() {
        let row = (program.pc - 1) / 40;
        let col = (program.pc - 1) % 40;

        if (program.x - col as isize).abs() <= 1 {
            crt[row][col] = '#';
        }

        program.exec_next();
    }

    return crt
        .iter()
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n");
}

fn main() {
    let input = include_str!("input/day10.input");
    println!("part1: {}", part1(input));
    println!("part2:\n{}", part2(input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let calories = include_str!("input/day10.test.input");

        assert_eq!(part1(calories), 13140);
    }

    #[test]
    fn test_part2() {
        let calories = include_str!("input/day10.test.input");

        let result = "\
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";

        assert_eq!(part2(calories), result);
    }
}
