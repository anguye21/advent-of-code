use std::fs;

fn main() {
    let filename = "assets/input-real.txt";

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let data: Vec<&str> = contents.lines().collect();

    let mut last_three: [i32; 3] = [0, 0, 0];

    let mut last = -1;
    let mut increase_count = 0;

    for (idx, depth_str) in data.iter().enumerate() {
        let depth: i32 = depth_str.parse().unwrap();
        last_three[idx % 3] = depth;

        if idx < 2 {
            continue;
        }

        let sliding_depth: i32 = last_three.iter().sum();

        if last != -1 && sliding_depth > last {
            increase_count += 1;
        }

        last = sliding_depth;
    }

    println!("{}", increase_count);
}
