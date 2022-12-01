use std::fs;

fn main() {
    let filename = "assets/input-real.txt";

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let data: Vec<&str> = contents.lines().collect();
    let len: usize = data.len();

    let mut epsilon_rate: i32 = 0;
    let mut gamma_rate: i32 = 0;

    let mut ones_count: Vec<i32> = Vec::new();
    let bin_len: usize = data[0].chars().count();

    for _ in 0..bin_len {
        ones_count.push(0);
    }

    for line in data {
        for (idx, char) in line.chars().enumerate() {
            ones_count[idx] += (char == '1') as i32;
        }
    }

    for (idx, count) in ones_count.iter().enumerate() {
        let bit = (count >= &((len / 2) as i32)) as i32;
        let bit_pos = bin_len - idx - 1;
        epsilon_rate |= bit << bit_pos;
        gamma_rate |= (bit + 1) % 2 << bit_pos;
    }

    println!("er = {}", epsilon_rate);
    println!("gr = {}", gamma_rate);
    println!("\n{}", epsilon_rate * gamma_rate);
}
