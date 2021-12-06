use std::collections::HashMap;
use std::fs;

fn main() {
    let filename = "assets/input-real.txt";

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let data: Vec<&str> = contents.lines().collect();

    let mut num_lantern_fish: u64 = 0;
    let mut state: HashMap<i32, u64> = HashMap::new();

    for i in 0..9 {
        state.insert(i, 0);
    }

    for initial_timer in data[0].split(",") {
        *state
            .get_mut(&initial_timer.parse::<i32>().unwrap())
            .unwrap() += 1;
    }

    for _ in 0..256 {
        let num_new_fishes = state[&0];

        *state.get_mut(&0).unwrap() = state[&1];
        *state.get_mut(&1).unwrap() = state[&2];
        *state.get_mut(&2).unwrap() = state[&3];
        *state.get_mut(&3).unwrap() = state[&4];
        *state.get_mut(&4).unwrap() = state[&5];
        *state.get_mut(&5).unwrap() = state[&6];
        *state.get_mut(&6).unwrap() = state[&7] + num_new_fishes;
        *state.get_mut(&7).unwrap() = state[&8];
        *state.get_mut(&8).unwrap() = num_new_fishes;
    }

    for i in 0..9 {
        num_lantern_fish += state[&i];
    }

    println!("{}", num_lantern_fish);
}
