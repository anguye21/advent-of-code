use std::{str::FromStr, collections::VecDeque, cell::RefCell, rc::Rc};

use aoc2022::ParseInputError;

#[derive(Debug)]
struct Monkey {
    id: usize,
    items: VecDeque<usize>,
    operation: String,
    div_test: usize,
    true_action: usize,
    false_action: usize,
}

impl Monkey {
    pub fn calc_operation(&self, old: usize) -> usize {
        let left: usize;
        let right: usize;

        return if self.operation.contains("+") {
            let (l, r) = self.operation.split_once(" + ").unwrap();
            left = if l == "old" { old } else { l.parse::<usize>().unwrap() };
            right = if r == "old" { old } else { r.parse::<usize>().unwrap() };

            left + right
        } else if self.operation.contains("*") {
            let (l, r) = self.operation.split_once(" * ").unwrap();
            left = if l == "old" { old } else { l.parse::<usize>().unwrap() };
            right = if r == "old" { old } else { r.parse::<usize>().unwrap() };

            left * right
        } else {
            panic!("opertion not supported");
        }
    }
}

impl FromStr for Monkey {
    type Err = ParseInputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s
            .split("\n")
            .collect::<Vec<&str>>();

        let id = lines[0]
            .split_once(" ")
            .ok_or(ParseInputError)?
            .1;

        let id = id[..id.chars().count() - 1]
            .parse::<usize>()
            .map_err(|_| ParseInputError)?;

        let items = lines[1]
            .split_once(": ")
            .ok_or(ParseInputError)?
            .1
            .trim()
            .split(", ")
            .map(|item| item.parse::<usize>().unwrap())
            .collect::<VecDeque<usize>>();

        let operation = lines[2]
            .split_once(" = ")
            .ok_or(ParseInputError)?
            .1
            .to_owned();

        let div_test = lines[3]
            .split_once("Test: divisible by ")
            .ok_or(ParseInputError)?
            .1
            .parse::<usize>()
            .map_err(|_| ParseInputError)?;

        let true_action = lines[4]
            .split_once("If true: throw to monkey ")
            .ok_or(ParseInputError)?
            .1
            .parse::<usize>()
            .map_err(|_| ParseInputError)?;


        let false_action = lines[5]
            .split_once("If false: throw to monkey ")
            .ok_or(ParseInputError)?
            .1
            .parse::<usize>()
            .map_err(|_| ParseInputError)?;

        return Ok(Monkey {
            id,
            items,
            operation,
            div_test,
            true_action,
            false_action
        });
    }
}

fn part1(input: &str) -> usize {
    let mut num_inspections: Vec<usize> = Vec::new();
    let mut monkeys: Vec<Rc<RefCell<Monkey>>> = Vec::new();

    for monkey in input.split("\n\n") {
        monkeys.push(Rc::from(RefCell::from(monkey.parse::<Monkey>().unwrap())));
        num_inspections.push(0);
    }

    for round in 0..20 {
        for i in 0..monkeys.len() {
            let mut monkey = monkeys[i].borrow_mut();

            while monkey.items.len() > 0 {
                let mut next_item = monkey.items.pop_front().unwrap();
                next_item = monkey.calc_operation(next_item);
                next_item /= 3;

                let mut next_monkey = if next_item % monkey.div_test == 0 {
                    monkeys[monkey.true_action].borrow_mut()
                } else {
                    monkeys[monkey.false_action].borrow_mut()
                };

                next_monkey.items.push_back(next_item);
                num_inspections[i] += 1;
            }
        }
    }

    num_inspections.sort_by(|a, b| b.cmp(a));
    return num_inspections[0] * num_inspections[1];
}

fn part2(input: &str) -> usize {
    let mut num_inspections: Vec<usize> = Vec::new();
    let mut monkeys: Vec<Rc<RefCell<Monkey>>> = Vec::new();

    for monkey in input.split("\n\n") {
        monkeys.push(Rc::from(RefCell::from(monkey.parse::<Monkey>().unwrap())));
        num_inspections.push(0);
    }

    let common_divisor = monkeys.iter().fold(1, |acc, x| acc*x.borrow().div_test);

    for round in 0..10000 {
        for i in 0..monkeys.len() {
            let mut monkey = monkeys[i].borrow_mut();

            while monkey.items.len() > 0 {
                let mut next_item = monkey.items.pop_front().unwrap();
                next_item = monkey.calc_operation(next_item);
                next_item %= common_divisor;

                let mut next_monkey = if next_item % monkey.div_test == 0 {
                    monkeys[monkey.true_action].borrow_mut()
                } else {
                    monkeys[monkey.false_action].borrow_mut()
                };

                next_monkey.items.push_back(next_item);
                num_inspections[i] += 1;
            }
        }
    }

    num_inspections.sort_by(|a, b| b.cmp(a));
    return num_inspections[0] * num_inspections[1];
}

fn main() {
    let input = include_str!("./input/day11.input");
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "\
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

        assert_eq!(part1(input), 10605);
    }


    #[test]
    fn test_part2() {
        let input = "\
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

        assert_eq!(part2(input), 2713310158);
    }
}
