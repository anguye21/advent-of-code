use std::{fs, str::FromStr};

#[derive(Debug, Clone)]
enum SnailFishNumber {
    Literal(i32),
    Pair(Box<SnailFishNumber>, Box<SnailFishNumber>),
}

impl FromStr for SnailFishNumber {
    type Err = std::string::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn find_comma_split_idx(s: &str) -> usize {
            let mut nesting = 0;

            for (idx, c) in s.chars().enumerate() {
                if c == '[' {
                    nesting += 1;
                } else if c == ']' {
                    nesting -= 1;
                } else if c == ',' && nesting == 1 {
                    return idx;
                }
            }

            return 0;
        }

        fn parse_str(s: &str) -> SnailFishNumber {
            if s.chars().all(|c| c.is_numeric() || c == '-') {
                return SnailFishNumber::Literal(s.parse().unwrap());
            }

            let split_idx = find_comma_split_idx(s);
            let left = &s[1..split_idx];
            let right = &s[split_idx + 1..s.len() - 1];

            return SnailFishNumber::Pair(Box::new(parse_str(left)), Box::new(parse_str(right)));
        }

        return Ok(parse_str(s));
    }
}

fn add_left(m: SnailFishNumber, n: Option<SnailFishNumber>) -> SnailFishNumber {
    if n.is_none() {
        return m;
    }

    if let SnailFishNumber::Literal(m_lit) = m {
        if let SnailFishNumber::Literal(n_lit) = n.unwrap() {
            return SnailFishNumber::Literal(m_lit + n_lit);
        } else {
            unreachable!();
        }
    }

    if let SnailFishNumber::Pair(l, r) = m {
        return SnailFishNumber::Pair(Box::new(add_left(*l, n)), r);
    }

    panic!();
}

fn add_right(m: SnailFishNumber, n: Option<SnailFishNumber>) -> SnailFishNumber {
    if n.is_none() {
        return m;
    }

    if let SnailFishNumber::Literal(m_lit) = m {
        if let SnailFishNumber::Literal(n_lit) = n.unwrap() {
            return SnailFishNumber::Literal(m_lit + n_lit);
        } else {
            unreachable!();
        }
    }

    if let SnailFishNumber::Pair(l, r) = m {
        return SnailFishNumber::Pair(l, Box::new(add_right(*r, n)));
    }

    panic!();
}

fn explode(
    sfn: &SnailFishNumber,
    nesting: u32,
) -> (
    bool,
    Option<SnailFishNumber>,
    Option<SnailFishNumber>,
    SnailFishNumber,
) {
    if let SnailFishNumber::Literal(_) = *sfn {
        return (false, None, None, sfn.clone());
    } else if let SnailFishNumber::Pair(a, b) = sfn.clone() {
        if nesting == 4 {
            return (true, Some(*a), Some(*b), SnailFishNumber::Literal(0));
        }

        let (exploded, left, right, a) = explode(a.as_ref(), nesting + 1);

        if exploded {
            return (
                true,
                left,
                None,
                SnailFishNumber::Pair(Box::new(a), Box::new(add_left(b.as_ref().clone(), right))),
            );
        }

        let (exploded, left, right, b) = explode(b.as_ref(), nesting + 1);

        if exploded {
            return (
                true,
                None,
                right,
                SnailFishNumber::Pair(Box::new(add_right(a, left)), Box::new(b.clone())),
            );
        }
    }

    return (false, None, None, sfn.clone());
}

fn split(sfn: &mut SnailFishNumber) -> bool {
    if let SnailFishNumber::Literal(n) = sfn {
        if *n >= 10 {
            let left = ((*n as f32) / 2.0).floor() as i32;
            let right = ((*n as f32) / 2.0).ceil() as i32;

            *sfn = SnailFishNumber::Pair(
                Box::new(SnailFishNumber::Literal(left)),
                Box::new(SnailFishNumber::Literal(right)),
            );

            return true;
        }
    } else if let SnailFishNumber::Pair(left, right) = sfn {
        return split(left) || split(right);
    }
    return false;
}

fn reduce(sfn: &mut SnailFishNumber) {
    loop {
        let (change, _, _, sfn_new) = explode(sfn, 0);

        *sfn = sfn_new;

        if change {
            continue;
        }

        if !split(sfn) {
            break;
        }
    }
}

fn add(lhs: SnailFishNumber, rhs: SnailFishNumber) -> SnailFishNumber {
    let mut add = SnailFishNumber::Pair(Box::new(lhs), Box::new(rhs));
    reduce(&mut add);
    return add;
}

fn magnitude(num: SnailFishNumber) -> i32 {
    return match num {
        SnailFishNumber::Literal(n) => n,
        SnailFishNumber::Pair(l, r) => 3 * magnitude(*l) + 2 * magnitude(*r),
    };
}

fn main() {
    let filename = "assets/input-real.txt";

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let data: Vec<&str> = contents.lines().collect();

    let added: SnailFishNumber = data
        .iter()
        .map(|s| s.parse::<SnailFishNumber>().unwrap())
        .reduce(|a, b| add(a, b))
        .unwrap();

    println!("{:?}", magnitude(added));
}
