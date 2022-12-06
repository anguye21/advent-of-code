use std::collections::HashMap;

fn solution(input: &str, distinct: usize) -> usize {
    let mut freq: HashMap<char, i32> = HashMap::new();

    for i in 0..distinct {
        freq.entry(input.chars().nth(i).unwrap()).and_modify(|f| *f += 1).or_insert(1);
    }

    if freq.len() == distinct {
        return distinct;
    }

    for end in distinct..input.len() {
        let start = end - distinct;
        let start_key = input.chars().nth(start).unwrap();
        let end_key = input.chars().nth(end).unwrap();

        freq.entry(end_key).and_modify(|f| *f += 1).or_insert(1);
        freq.entry(start_key).and_modify(|f| *f -= 1);

        if freq[&start_key] == 0 {
            freq.remove(&start_key);
        }

        if freq.len() == distinct {
            return end + 1;
        }
    }

    return input.len();
}

fn main() {
    let input = include_str!("./input/day6.input");
    println!("part1: {}", solution(input, 4));
    println!("part2: {}", solution(input, 14));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(solution("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4), 7);
        assert_eq!(solution("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5);
        assert_eq!(solution("nppdvjthqldpwncqszvftbrmjlhg", 4), 6);
        assert_eq!(solution("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4), 10);
        assert_eq!(solution("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4), 11);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solution("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14), 19);
        assert_eq!(solution("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), 23);
        assert_eq!(solution("nppdvjthqldpwncqszvftbrmjlhg", 14), 23);
        assert_eq!(solution("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14), 29);
        assert_eq!(solution("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14), 26);
    }
}
