use std::collections::HashSet;
use std::fs;

fn main() {
    let filename = "assets/input-real.txt";

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let data: Vec<&str> = contents.lines().collect();

    let mut sum = 0;

    for line in data {
        let x: Vec<&str> = line.split(" | ").collect();

        let inputs = x[0];
        let outputs = x[1];

        let mut one: HashSet<char> = HashSet::new();
        let mut four: HashSet<char> = HashSet::new();

        for input in inputs.split(" ") {
            let num_segments = input.len() as u32;

            match num_segments {
                2 => one = HashSet::from_iter(input.chars()),
                4 => four = HashSet::from_iter(input.chars()),
                _ => continue,
            }
        }

        let mut factor = 1000;

        for output in outputs.split(" ") {
            let num_segments = output.len() as u32;
            let num;

            let hs = HashSet::from_iter(output.chars());
            let one_intersection = hs.intersection(&one);
            let four_intersection = hs.intersection(&four);

            match num_segments {
                2 => num = 1,
                3 => num = 7,
                4 => num = 4,
                5 => {
                    if one_intersection.count() == 2 {
                        num = 3;
                    } else if four_intersection.count() == 3 {
                        num = 5;
                    } else {
                        num = 2;
                    }
                }
                6 => {
                    if one_intersection.count() == 1 {
                        num = 6;
                    } else if four_intersection.count() == 4 {
                        num = 9;
                    } else {
                        num = 0;
                    }
                }
                7 => num = 8,
                _ => panic!(),
            }

            sum += num * factor;
            factor /= 10;
        }
    }

    println!("{}", sum);
}
