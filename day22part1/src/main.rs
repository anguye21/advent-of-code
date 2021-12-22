use std::{collections::HashSet, fs};

fn main() {
    let filename = "assets/input-real.txt";

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let data: Vec<&str> = contents.lines().collect();

    let mut cubes: HashSet<(i32, i32, i32)> = HashSet::new();

    let steps: Vec<(String, (i32, i32), (i32, i32), (i32, i32))> = data
        .iter()
        .map(|line| {
            let state = line.chars().take_while(|c| *c != ' ').collect::<String>();

            let mut x_range = (0, 0);
            x_range.0 = line
                .chars()
                .skip_while(|c| *c != 'x')
                .skip_while(|c| !c.is_numeric() && *c != '-')
                .take_while(|c| c.is_numeric() || *c == '-')
                .collect::<String>()
                .parse::<i32>()
                .unwrap();
            x_range.1 = line
                .chars()
                .skip_while(|c| *c != 'x')
                .skip_while(|c| *c != '.')
                .skip(2)
                .take_while(|c| c.is_numeric() || *c == '-')
                .collect::<String>()
                .parse::<i32>()
                .unwrap();

            let mut y_range = (0, 0);
            y_range.0 = line
                .chars()
                .skip_while(|c| *c != 'y')
                .skip_while(|c| !c.is_numeric() && *c != '-')
                .take_while(|c| c.is_numeric() || *c == '-')
                .collect::<String>()
                .parse::<i32>()
                .unwrap();
            y_range.1 = line
                .chars()
                .skip_while(|c| *c != 'y')
                .skip_while(|c| *c != '.')
                .skip(2)
                .take_while(|c| c.is_numeric() || *c == '-')
                .collect::<String>()
                .parse::<i32>()
                .unwrap();

            let mut z_range = (0, 0);
            z_range.0 = line
                .chars()
                .skip_while(|c| *c != 'z')
                .skip_while(|c| !c.is_numeric() && *c != '-')
                .take_while(|c| c.is_numeric() || *c == '-')
                .collect::<String>()
                .parse::<i32>()
                .unwrap();
            z_range.1 = line
                .chars()
                .skip_while(|c| *c != 'z')
                .skip_while(|c| *c != '.')
                .skip(2)
                .take_while(|c| c.is_numeric() || *c == '-')
                .collect::<String>()
                .parse::<i32>()
                .unwrap();

            return (state, x_range, y_range, z_range);
        })
        .collect();

    for step in steps {
        if -50 > step.1.1 || 50 < step.1.0 ||
           -50 > step.2.1 || 50 < step.2.0 ||
           -50 > step.3.1 || 50 < step.3.0
        {
            continue;
        }

        for x in step.1.0.max(-50)..=step.1.1.min(50) {
            for y in step.2.0.max(-50)..=step.2.1.min(50) {
                for z in step.3.0.max(-50)..=step.3.1.min(50) {
                    if step.0 == "on" {
                        cubes.insert((x, y, z));
                    } else {
                        cubes.remove(&(x, y, z));
                    }
                }
            }
        }
    }



    println!("{}", cubes.len());
}
