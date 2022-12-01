use std::fs;

fn main() {
    let filename = "assets/input-real.txt";

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let data: Vec<&str> = contents.lines().collect();

    let mut horizontal: i32 = 0;
    let mut depth: i32 = 0;
    let mut aim: i32 = 0;

    for command in data {
        let command_vec: Vec<&str> = command.split(" ").collect();
        let direction: &str = command_vec[0];
        let amount: i32 = command_vec[1].parse().unwrap();

        match direction {
            "forward" => {
                horizontal += amount;
                depth += aim * amount;
            }
            "down" => aim += amount,
            "up" => aim -= amount,
            &_ => (),
        }
    }

    println!("horizontal = {}", horizontal);
    println!("depth = {}", depth);

    println!("\n{}", horizontal * depth);
}
