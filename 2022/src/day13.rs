use std::cmp;

use serde_json::Value;

fn compare(left: &Value, right: &Value) -> Option<bool> {
    match (left, right) {
        (Value::Number(l), Value::Number(r)) => {
            if l.as_i64() == r.as_i64() {
                return None;
            }

            Some(l.as_i64() < r.as_i64())
        },
        (Value::Array(l), Value::Array(r)) => {
            for i in 0..cmp::min(l.len(), r.len()) {
                if let Some(res) = compare(&l[i], &r[i]) {
                    return Some(res);
                }
            }

            if l.len() > r.len() {
                return Some(false);
            } else if r.len() > l.len() {
                return Some(true);
            }

            None
        },
        (Value::Array(_), Value::Number(r)) => {
            compare(left, &Value::Array(vec![Value::Number(r.clone())]))
        },
        (Value::Number(l), Value::Array(_)) => {
            compare(&Value::Array(vec![Value::Number(l.clone())]), right)
        },
        _ => unreachable!(),
    }
}

fn part1(input: &str) -> usize {
    let pairs = input.split("\n\n");
    let mut sum = 0;

    for (i, pair) in pairs.enumerate() {
        let (l, r) = pair.split_once("\n").unwrap();
        let left = serde_json::from_str(l).unwrap();
        let right = serde_json::from_str(r).unwrap();

        if compare(&left, &right).unwrap_or_else(|| true) {
            sum += i + 1;
        }
    }


    return sum;
}

fn part2(input: &str) -> usize {
    let pairs = input.split("\n\n");
    let two_div: Value = serde_json::from_str("[[2]]").unwrap();
    let six_div: Value = serde_json::from_str("[[6]]").unwrap();
    let mut packets = vec![two_div.clone(), six_div.clone()];

    for pair in pairs {
        let (l, r) = pair.split_once("\n").unwrap();
        let left = serde_json::from_str(l).unwrap();
        let right = serde_json::from_str(r).unwrap();

        packets.push(left);
        packets.push(right);
    }

    packets.sort_by(|a, b| if compare(&a, &b).unwrap_or_else(|| true) {
        cmp::Ordering::Less
    } else {
        cmp::Ordering::Greater
    });

    let mut res = 1;

    for (i, packet) in packets.iter().enumerate() {
        if compare(&packet, &two_div) == None || compare(&packet, &six_div) == None {
            res *= i+1;
        }
    }

    return res;
}

fn main() {
    let input = include_str!("./input/day13.input");
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "\
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

        assert_eq!(part1(input), 13);
    }

    #[test]
    fn test_part2() {
        let input = "\
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

        assert_eq!(part2(input), 140);

    }
}
