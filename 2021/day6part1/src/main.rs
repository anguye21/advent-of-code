use std::collections::LinkedList;
use std::fs;

fn main() {
    let filename = "assets/input-real.txt";

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let data: Vec<&str> = contents.lines().collect();

    let mut state: LinkedList<i32> = LinkedList::new();

    for initial_timer in data[0].split(",") {
        state.push_back(initial_timer.parse().unwrap());
    }

    for _ in 0..80 {
        let mut num_new_lantern_fish = 0;

        for timer in state.iter_mut() {
            if *timer == 0 {
                num_new_lantern_fish += 1;
                *timer = 6;
            } else {
                *timer -= 1;
            }
        }

        for _ in 0..num_new_lantern_fish {
            state.push_back(8);
        }
    }

    println!("{}", state.len());
}
