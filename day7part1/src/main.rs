use std::collections::HashMap;
use std::fs;

fn main() {
    let filename = "assets/input-real.txt";

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let data: Vec<&str> = contents.lines().collect();

    let positions: Vec<i32> = data[0]
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let mut fuel_usage: HashMap<i32, i32> = HashMap::new();

    for end_position in &positions {
        if fuel_usage.contains_key(&end_position) {
            continue;
        }

        let mut fuel_cost = 0;

        for pos in &positions {
            fuel_cost += (end_position - pos).abs();
        }

        fuel_usage.insert(*end_position, fuel_cost);
    }

    let min_fuel_usage = fuel_usage
        .into_iter()
        .reduce(|a, b| if a.1 < b.1 { a } else { b })
        .unwrap()
        .1;

    println!("{}", min_fuel_usage);
}
