use std::{collections::HashMap, fs};

fn apply_rules(curr_polymer: &mut Vec<char>, rules: &Vec<((char, char), char)>) {
    let mut insertions: Vec<(char, usize)> = Vec::new();

    for rule in rules {
        for idx in 0..(curr_polymer.len() - 1) {
            if curr_polymer[idx] == rule.0 .0 && curr_polymer[idx + 1] == rule.0 .1 {
                insertions.push((rule.1, idx));
            }
        }
    }

    insertions.sort_by(|a, b| a.1.cmp(&b.1));

    for (idx, insertion) in insertions.iter().enumerate() {
        curr_polymer.insert(insertion.1 + idx + 1, insertion.0);
    }
}

fn main() {
    let filename = "assets/input-real.txt";

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let data: Vec<&str> = contents.lines().collect();

    let mut curr_polymer = data[0].chars().collect::<Vec<char>>();
    let mut occurances: HashMap<char, usize> = HashMap::new();

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

    for _ in 0..10 {
        apply_rules(&mut curr_polymer, &rules);
    }

    for element in &curr_polymer {
        let element_count = occurances.entry(*element).or_insert(0);
        *element_count += 1;
    }

    let min = occurances
        .iter()
        .reduce(|accum, item| if accum.1 < item.1 { accum } else { item })
        .unwrap()
        .1;

    let max = occurances
        .iter()
        .reduce(|accum, item| if accum.1 > item.1 { accum } else { item })
        .unwrap()
        .1;

    println!("{}", max - min);
}
