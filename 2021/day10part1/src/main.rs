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

    let mut illegal_closing_poins: HashMap<char, u32> = HashMap::new();

    illegal_closing_poins.insert(')', 3);
    illegal_closing_poins.insert(']', 57);
    illegal_closing_poins.insert('}', 1197);
    illegal_closing_poins.insert('>', 25137);

    let opening_chunks: Vec<char> = chunk_pairs.keys().cloned().collect();
    let closing_chunks: Vec<char> = chunk_pairs.values().cloned().collect();

    let mut score: u32 = 0;

    for line in data {
        let mut chunks: LinkedList<char> = LinkedList::new();
        for c in line.chars() {
            if opening_chunks.contains(&c) {
                chunks.push_front(c);
            } else if closing_chunks.contains(&c) {
                if chunk_pairs[&chunks.pop_front().unwrap()] != c {
                    score += illegal_closing_poins[&c];
                }
            } else {
                panic!();
            }
        }
    }

    println!("{}", score);
}
