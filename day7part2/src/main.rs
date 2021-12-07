use std::fs;

fn main() {
    let filename = "assets/input-real.txt";

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let data: Vec<&str> = contents.lines().collect();

    let positions: Vec<i32> = data[0]
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let min_pos = positions.iter().min().unwrap();
    let max_pos = positions.iter().max().unwrap();
    let mut curr_min_fuel_cost = -1;

    for end_position in *min_pos..*max_pos + 1 {
        let mut fuel_cost = 0;

        for pos in &positions {
            let n = (end_position - pos).abs();
            fuel_cost += n * (n + 1) / 2;
        }

        if fuel_cost < curr_min_fuel_cost || curr_min_fuel_cost == -1 {
            curr_min_fuel_cost = fuel_cost;
        }
    }

    println!("{}", curr_min_fuel_cost);
}
