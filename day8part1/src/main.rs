use std::fs;

fn main() {
    let filename = "assets/input-real.txt";

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let data: Vec<&str> = contents.lines().collect();

    let mut num_unique = 0;

    for line in data {
        let outputs: &str = line.split(" | ").collect::<Vec<&str>>()[1];

        for output in outputs.split(" ") {
            if [2, 4, 3, 7].contains(&(output.len() as u32)) {
                num_unique += 1;
            }
        }
    }
    println!("{}", num_unique);
}
