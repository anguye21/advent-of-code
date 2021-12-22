use std::fs;

struct Player {
    position: u32,
    score: u32,
}

fn main() {
    let filename = "assets/input-real.txt";

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let data: Vec<&str> = contents.lines().collect();

    let mut player_1 = Player {
        position: data[0]
            .chars()
            .skip_while(|c| *c != ':')
            .skip(2)
            .fold(0, |accum, c| accum * 10 + c.to_digit(10).unwrap()),
        score: 0,
    };

    let mut player_2 = Player {
        position: data[1]
            .chars()
            .skip_while(|c| *c != ':')
            .skip(2)
            .fold(0, |accum, c| accum * 10 + c.to_digit(10).unwrap()),
        score: 0,
    };

    let mut turns = 0u32;
    let mut next_roll = 1u32;

    loop {
        for _ in 0..3 {
            player_1.position = player_1.position + next_roll;
            next_roll = (next_roll % 100) + 1;
        }

        player_1.position = (player_1.position - 1) % 10 + 1;
        player_1.score += player_1.position;


        turns += 3;

        if player_1.score >= 1000 {
            break;
        }


        for _ in 0..3 {
            player_2.position = player_2.position + next_roll;
            next_roll = (next_roll % 100) + 1;
        }

        player_2.position = (player_2.position - 1) % 10 + 1;
        player_2.score += player_2.position;

        turns += 3;

        if player_2.score >= 1000 {
            break;
        }

    }

    println!("{}", [player_1.score, player_2.score].iter().min().unwrap() * turns);
}
