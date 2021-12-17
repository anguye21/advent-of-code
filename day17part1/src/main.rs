use std::fs;

fn step(x_pos: &mut i32, y_pos: &mut i32, x_vel: &mut i32, y_vel: &mut i32) {
    *x_pos += *x_vel;
    *y_pos += *y_vel;
    *y_vel -= 1;
    *x_vel -= if *x_vel > 0 {
        1
    } else if *x_vel < 0 {
        -1
    } else {
        0
    };
}

fn past_range(x_pos: i32, y_pos: i32, x_max: i32, y_min: i32) -> bool {
    return x_pos > x_max || y_pos < y_min;
}

fn in_range(x_pos: i32, y_pos: i32, x_range: (i32, i32), y_range: (i32, i32)) -> bool {
    return (x_range.0 <= x_pos && x_pos <= x_range.1)
        && (y_range.0 <= y_pos && y_pos <= y_range.1);
}

fn check_in_range(vel: (i32, i32), x_range: (i32, i32), y_range: (i32, i32)) -> i32 {
    let mut x_pos = 0;
    let mut y_pos = 0;
    let mut x_vel = vel.0;
    let mut y_vel = vel.1;

    let mut max_height = std::i32::MIN;

    while !past_range(x_pos, y_pos, x_range.1, y_range.0)
        && !in_range(x_pos, y_pos, x_range, y_range)
    {
        step(&mut x_pos, &mut y_pos, &mut x_vel, &mut y_vel);
        max_height = max_height.max(y_pos);
    }

    if in_range(x_pos, y_pos, x_range, y_range) {
        return max_height;
    }

    return std::i32::MIN;
}

fn find_highest(x_range: (i32, i32), y_range: (i32, i32)) -> i32 {
    let mut max_height = std::i32::MIN;

    for x_vel in -x_range.0..=x_range.1 {
        for y_vel in y_range.0..=-y_range.0 {
            let height = check_in_range((x_vel, y_vel), x_range, y_range);

            max_height = max_height.max(height);
        }
    }

    return max_height;
}

fn main() {
    let filename = "assets/input-real.txt";

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let data: Vec<&str> = contents.lines().collect();

    let x_range_str = data[0]
        .chars()
        .skip_while(|c| !c.is_digit(10) && *c != '-')
        .take_while(|c| *c != ',')
        .collect::<String>();

    let x_range = [x_range_str.split_once("..").unwrap()]
        .iter()
        .map(|(x1, x2)| (x1.parse::<i32>().unwrap(), x2.parse::<i32>().unwrap()))
        .collect::<Vec<(i32, i32)>>()[0];

    let y_range_str = data[0]
        .chars()
        .skip_while(|c| *c != 'y')
        .skip_while(|c| !c.is_digit(10) && *c != '-')
        .collect::<String>();

    let y_range = [y_range_str.split_once("..").unwrap()]
        .iter()
        .map(|(y1, y2)| (y1.parse::<i32>().unwrap(), y2.parse::<i32>().unwrap()))
        .collect::<Vec<(i32, i32)>>()[0];

    println!("{}", find_highest(x_range, y_range));
}
