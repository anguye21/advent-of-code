use std::fs;

fn main() {
    let filename = "assets/input-real.txt";

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let data: Vec<&str> = contents.lines().collect();

    let mut oxygen_generator_rating_candidates: Vec<&str> = data.to_vec();
    let mut co2_scrubber_rating_candidates: Vec<&str> = data.to_vec();

    let bin_len: usize = data[0].chars().count();

    for bit_pos in 0..bin_len {
        let mut ones_count: i32 = 0;
        let mut zeros_count: i32 = 0;

        for binary in oxygen_generator_rating_candidates.iter() {
            let c = binary.chars().nth(bit_pos).unwrap();
            ones_count += (c == '1') as i32;
            zeros_count += (c == '0') as i32;
        }

        let most_common_bit = (ones_count >= zeros_count) as i32;
        oxygen_generator_rating_candidates = oxygen_generator_rating_candidates
            .iter()
            .filter(|candidate| {
                let bit = (candidate.chars().nth(bit_pos).unwrap() == '1') as i32;
                return bit == most_common_bit;
            })
            .cloned()
            .collect();

        if oxygen_generator_rating_candidates.len() == 1 {
            break;
        }
    }

    for bit_pos in 0..bin_len {
        let mut ones_count: i32 = 0;
        let mut zeros_count: i32 = 0;

        for binary in co2_scrubber_rating_candidates.iter() {
            let c = binary.chars().nth(bit_pos).unwrap();
            ones_count += (c == '1') as i32;
            zeros_count += (c == '0') as i32;
        }

        let most_common_bit = (ones_count >= zeros_count) as i32;
        co2_scrubber_rating_candidates = co2_scrubber_rating_candidates
            .iter()
            .filter(|candidate| {
                let bit = (candidate.chars().nth(bit_pos).unwrap() == '1') as i32;
                return bit != most_common_bit;
            })
            .cloned()
            .collect();

        if co2_scrubber_rating_candidates.len() == 1 {
            break;
        }
    }

    let mut oxygen_generator_rating: i32 = 0;
    let mut co2_scrubber_rating: i32 = 0;

    for idx in 0..oxygen_generator_rating_candidates[0].chars().count() {
        let bit_pos = oxygen_generator_rating_candidates[0].chars().count() - idx - 1;
        let bit = oxygen_generator_rating_candidates[0]
            .chars()
            .nth(idx)
            .unwrap();
        oxygen_generator_rating |= ((bit == '1') as i32) << bit_pos;
    }

    for idx in 0..co2_scrubber_rating_candidates[0].chars().count() {
        let bit_pos = co2_scrubber_rating_candidates[0].chars().count() - idx - 1;
        let bit = co2_scrubber_rating_candidates[0].chars().nth(idx).unwrap();
        co2_scrubber_rating |= ((bit == '1') as i32) << bit_pos;
    }

    println!("oxygen = {}", oxygen_generator_rating);
    println!("co2 = {}", co2_scrubber_rating);
    println!("\n{}", oxygen_generator_rating * co2_scrubber_rating);
}
