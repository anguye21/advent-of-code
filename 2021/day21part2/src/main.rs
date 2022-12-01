use std::{collections::HashMap, fs};

fn simulate(
    pos_1: u32,
    pos_2: u32,
    points_1: u32,
    points_2: u32,
    num_rolls_to_win: &mut HashMap<(u32, u32, u32, u32), (u64, u64)>,
) -> (u64, u64) {
    if points_1 >= 21 {
        return (1, 0);
    }

    if points_2 >= 21 {
        return (0, 1);
    }

    if num_rolls_to_win.contains_key(&(pos_1, pos_2, points_1, points_2)) {
        return num_rolls_to_win[&(pos_1, pos_2, points_1, points_2)];
    }

    let mut wins_1 = 0u64;
    let mut wins_2 = 0u64;

    let roll_sums = [
        3, 4, 4, 4, 5, 5, 5, 5, 5, 5, 6, 6, 6, 6, 6, 6, 6, 7, 7, 7, 7, 7, 7, 8, 8, 8, 9,
    ];

    for rolls_sum_1 in &roll_sums {
        for rolls_sum_2 in &roll_sums {
            let new_pos_1 = (pos_1 + rolls_sum_1 - 1) % 10 + 1;
            let new_pos_2 = (pos_2 + rolls_sum_2 - 1) % 10 + 1;
            let (win_1, win_2) = simulate(
                new_pos_1,
                new_pos_2,
                points_1 + new_pos_1,
                points_2 + new_pos_2,
                num_rolls_to_win,
            );

            wins_1 += win_1;
            wins_2 += win_2;
        }
    }

    num_rolls_to_win.insert((pos_1, pos_2, points_1, points_2), (wins_1, wins_2));

    return (wins_1, wins_2);
}

fn num_wins(player_1_start: u32, player_2_start: u32) -> (u64, u64) {
    let mut num_rolls_to_win: HashMap<(u32, u32, u32, u32), (u64, u64)> = HashMap::new();
    let result = simulate(player_1_start, player_2_start, 0, 0, &mut num_rolls_to_win);

    return (
        result.0 , // IDK why I have to do this, probably has something to do with the loop (roll_sums has 27 values)
        result.1,
    );
}

fn main() {
    let filename = "assets/input-real.txt";

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let data: Vec<&str> = contents.lines().collect();

    let player_1 = data[0]
        .chars()
        .skip_while(|c| *c != ':')
        .skip(2)
        .fold(0, |accum, c| accum * 10 + c.to_digit(10).unwrap());

    let player_2 = data[1]
        .chars()
        .skip_while(|c| *c != ':')
        .skip(2)
        .fold(0, |accum, c| accum * 10 + c.to_digit(10).unwrap());

    let (player_1_wins, player_2_wins) = num_wins(player_1, player_2);

    println!("{}", player_1_wins.max(player_2_wins));
}
