use std::{
    collections::{HashMap, LinkedList},
    fs,
};

fn main() {
    let filename = "assets/input-real.txt";

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let data: Vec<&str> = contents.lines().collect();

    let mut chunk_pairs: HashMap<char, char> = HashMap::new();

    chunk_pairs.insert('(', ')');
    chunk_pairs.insert('[', ']');
    chunk_pairs.insert('{', '}');
    chunk_pairs.insert('<', '>');

    let mut closing_points: HashMap<char, u64> = HashMap::new();
    let mut line_scores: Vec<u64> = Vec::new();

    closing_points.insert(')', 1);
    closing_points.insert(']', 2);
    closing_points.insert('}', 3);
    closing_points.insert('>', 4);

    let opening_chunks: Vec<char> = chunk_pairs.keys().cloned().collect();
    let closing_chunks: Vec<char> = chunk_pairs.values().cloned().collect();

    for line in data {
        let mut chunks: LinkedList<char> = LinkedList::new();
        let mut line_score = 0;
        let mut illegal = false;

        for c in line.chars() {
            if opening_chunks.contains(&c) {
                chunks.push_front(c);
            } else if closing_chunks.contains(&c) {
                if chunk_pairs[&chunks.pop_front().unwrap()] != c {
                    illegal = true;
                    break;
                }
            } else {
                panic!();
            }
        }

        while !chunks.is_empty() && !illegal {
            line_score *= 5;
            line_score += closing_points[&chunk_pairs[&chunks.pop_front().unwrap()]];
        }

        if !illegal {
            line_scores.push(line_score);
        }
    }

    line_scores.sort();

    println!("{}", line_scores[line_scores.len() / 2]);
}
