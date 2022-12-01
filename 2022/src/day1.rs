fn input_to_vec_of_vec(calories: &str) -> Vec<Vec<u32>> {
    return aoc2022::str_to_vec(calories, "\n")
        .iter()
        .map(|line| if line.trim() == "" { 0 } else { line.parse().unwrap() })
        .fold(Vec::new(), |mut acc: Vec<Vec<u32>>, x| {
            if x == 0 || acc.is_empty() {
                acc.push(Vec::new());
            }

            acc.last_mut().unwrap().push(x);
            return acc;
        });
}

fn part1(calories: &str) -> u32 {
    let sep_cals = input_to_vec_of_vec(calories);

    return sep_cals
        .iter()
        .map(|elf| elf.iter().sum())
        .max()
        .unwrap();
}

fn part2(calories: &str) -> u32 {
    let sep_cals = input_to_vec_of_vec(calories);

    let mut cal_totals = sep_cals
        .iter()
        .map(|elf| elf.iter().sum())
        .collect::<Vec<u32>>();

    cal_totals.sort_by(|a, b| b.cmp(a));

    return cal_totals[0] + cal_totals[1] + cal_totals[2];
}

fn main() {
    let input = include_str!("input/day1.input");
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let calories = "\
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
        ";

        assert_eq!(part1(calories), 24000);
    }

    #[test]
    fn test_part2() {
        let calories = "\
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
        ";

        assert_eq!(part2(calories), 45000);
    }
}
