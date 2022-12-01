use std::fs;

fn main() {
    let filename = "assets/input-real.txt";

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let data: Vec<&str> = contents.lines().collect();

    let mut last = -1;
    let mut increase_count = 0;

    for depth_str in data {
        let depth: i32 = depth_str.parse().unwrap();

        if last != -1 && depth > last {
            increase_count += 1;
        }

        last = depth;
    }

    println!("{}", increase_count);
}
