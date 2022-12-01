use std::{collections::HashMap, fs};

fn apply_rules(
    element_pair: &mut HashMap<(char, char), usize>,
    element_count: &mut HashMap<char, usize>,
    rules: &Vec<((char, char), char)>,
) {
    let mut changes: HashMap<(char, char), usize> = HashMap::new();

    for rule in rules {
        if element_pair.contains_key(&rule.0) && element_pair[&rule.0] != 0 {
            let curr_count = element_pair[&rule.0];

            *element_pair.entry(rule.0).or_insert(0) = 0;
            *changes.entry((rule.0 .0, rule.1)).or_insert(0) += curr_count;
            *changes.entry((rule.1, rule.0 .1)).or_insert(0) += curr_count;
            *element_count.entry(rule.1).or_insert(0) += curr_count;
        }
    }

    for (pair, change) in changes {
        *element_pair.entry(pair).or_insert(0) += change;
    }
}

fn main() {
    let filename = "assets/input-real.txt";

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let data: Vec<&str> = contents.lines().collect();

    let starting_polymer = data[0].chars().collect::<Vec<char>>();
    let mut element_pairs: HashMap<(char, char), usize> = HashMap::new();
    let mut element_count: HashMap<char, usize> = HashMap::new();

    let rules: Vec<((char, char), char)> = data
        .iter()
        .skip(2)
        .map(|rule| {
            let mut x = rule.splitn(2, " -> ");
            let mut pair = x.next().unwrap().chars();
            let insertion = x.next().unwrap().chars().nth(0).unwrap();

            return ((pair.next().unwrap(), pair.next().unwrap()), insertion);
        })
        .collect();

    let mut starting_polymer_iter = starting_polymer.iter();
    let mut prev = starting_polymer_iter.next();
    let mut curr = starting_polymer_iter.next();

    while curr != None {
        let first = prev.unwrap();
        let second = curr.unwrap();

        *element_count.entry(*first).or_insert(0) += 1;
        *element_pairs.entry((*first, *second)).or_insert(0) += 1;

        prev = curr;
        curr = starting_polymer_iter.next();
    }

    *element_count.entry(*prev.unwrap()).or_insert(0) += 1;

    for _ in 0..40 {
        apply_rules(&mut element_pairs, &mut element_count, &rules);
    }

    let min = element_count
        .iter()
        .reduce(|accum, item| if accum.1 < item.1 { accum } else { item })
        .map(|x| (*x.0, *x.1))
        .unwrap()
        .1;

    let max = element_count
        .iter()
        .reduce(|accum, item| if accum.1 > item.1 { accum } else { item })
        .map(|x| (*x.0, *x.1))
        .unwrap()
        .1;

    println!("{}", max - min);
}
