use std::collections::HashSet;

fn part1(input: &str) -> u32 {
    return input
        .lines()
        .map(|line| {
            let len = line.len() / 2;

            let left = &line[..len];
            let right = &line[len..];

            let left = HashSet::<char>::from_iter(left.chars());
            let right = HashSet::<char>::from_iter(right.chars());

            let item: char = *left.intersection(&right).next().unwrap();

            return if item.is_uppercase() {
                item as u32 - 'A' as u32 + 26
            } else {
                item as u32 - 'a' as u32
            } + 1;
        })
        .sum();
}

fn part2(input: &str) -> u32 {
    let mut lines = input.lines().peekable();
    let mut res = 0;

    while lines.peek().is_some() {
        let one = HashSet::<char>::from_iter(lines.next().unwrap().chars());
        let two = HashSet::<char>::from_iter(lines.next().unwrap().chars());
        let three = HashSet::<char>::from_iter(lines.next().unwrap().chars());

        let item: char = *one
            .intersection(&two)
            .map(|c| *c)
            .collect::<HashSet<char>>()
            .intersection(&three)
            .next()
            .unwrap();

        res += if item.is_ascii_uppercase() {
            item as u32 - 'A' as u32 + 26
        } else if item.is_ascii_lowercase() {
            item as u32 - 'a' as u32
        } else {
            unreachable!();
        } + 1;
    }

    return res;
}

fn main() {
    let input = include_str!("./input/day3.input");
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        assert_eq!(part1(input), 157);
    }


    #[test]
    fn test_part2() {
        let input = "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        assert_eq!(part2(input), 70);
    }
}
