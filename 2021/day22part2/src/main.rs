use std::fs;

#[derive(Clone)]
struct Cube {
    x1: i64,
    x2: i64,
    y1: i64,
    y2: i64,
    z1: i64,
    z2: i64,
}

fn main() {
    let filename = "assets/input-real.txt";

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let data: Vec<&str> = contents.lines().collect();

    let mut cubes: Vec<Option<Cube>> = Vec::new();

    let steps: Vec<(String, Cube)> = data
        .iter()
        .map(|line| {
            let state = line.chars().take_while(|c| *c != ' ').collect::<String>();

            let mut cube = Cube {
                x1: 0,
                x2: 0,
                y1: 0,
                y2: 0,
                z1: 0,
                z2: 0,
            };

            cube.x1 = line
                .chars()
                .skip_while(|c| *c != 'x')
                .skip_while(|c| !c.is_numeric() && *c != '-')
                .take_while(|c| c.is_numeric() || *c == '-')
                .collect::<String>()
                .parse::<i64>()
                .unwrap();
            cube.x2 = line
                .chars()
                .skip_while(|c| *c != 'x')
                .skip_while(|c| *c != '.')
                .skip(2)
                .take_while(|c| c.is_numeric() || *c == '-')
                .collect::<String>()
                .parse::<i64>()
                .unwrap();

            cube.y1 = line
                .chars()
                .skip_while(|c| *c != 'y')
                .skip_while(|c| !c.is_numeric() && *c != '-')
                .take_while(|c| c.is_numeric() || *c == '-')
                .collect::<String>()
                .parse::<i64>()
                .unwrap();
            cube.y2 = line
                .chars()
                .skip_while(|c| *c != 'y')
                .skip_while(|c| *c != '.')
                .skip(2)
                .take_while(|c| c.is_numeric() || *c == '-')
                .collect::<String>()
                .parse::<i64>()
                .unwrap();

            cube.z1 = line
                .chars()
                .skip_while(|c| *c != 'z')
                .skip_while(|c| !c.is_numeric() && *c != '-')
                .take_while(|c| c.is_numeric() || *c == '-')
                .collect::<String>()
                .parse::<i64>()
                .unwrap();
            cube.z2 = line
                .chars()
                .skip_while(|c| *c != 'z')
                .skip_while(|c| *c != '.')
                .skip(2)
                .take_while(|c| c.is_numeric() || *c == '-')
                .collect::<String>()
                .parse::<i64>()
                .unwrap();

            return (state, cube);
        })
        .collect();

    for step in 0..steps.len() {
        let cube1 = &steps[step].1;

        for c in 0..cubes.len() {
            if let Some(cube2) = cubes[c].clone() {
                if cube1.x1 > cube2.x2
                    || cube1.x2 < cube2.x1
                    || cube1.y1 > cube2.y2
                    || cube1.y2 < cube2.y1
                    || cube1.z1 > cube2.z2
                    || cube1.z2 < cube2.z1
                {
                    continue;
                }

                cubes[c] = None;

                if cube1.x1 > cube2.x1 {
                    cubes.push(Some(Cube {
                        x1: cube2.x1,
                        x2: cube1.x1 - 1,
                        ..cube2
                    }));
                }

                if cube1.x2 < cube2.x2 {
                    cubes.push(Some(Cube {
                        x1: cube1.x2 + 1,
                        ..cube2
                    }));
                }

                if cube1.y1 > cube2.y1 {
                    cubes.push(Some(Cube {
                        x1: cube1.x1.max(cube2.x1),
                        x2: cube1.x2.min(cube2.x2),
                        y2: cube1.y1 - 1,
                        ..cube2
                    }));
                }

                if cube1.y2 < cube2.y2 {
                    cubes.push(Some(Cube {
                        x1: cube1.x1.max(cube2.x1),
                        x2: cube1.x2.min(cube2.x2),
                        y1: cube1.y2 + 1,
                        ..cube2
                    }));
                }

                if cube1.z1 > cube2.z1 {
                    cubes.push(Some(Cube {
                        x1: cube1.x1.max(cube2.x1),
                        x2: cube1.x2.min(cube2.x2),
                        y1: cube1.y1.max(cube2.y1),
                        y2: cube1.y2.min(cube2.y2),
                        z1: cube2.z1,
                        z2: cube1.z1 - 1,
                    }));
                }

                if cube1.z2 < cube2.z2 {
                    cubes.push(Some(Cube {
                        x1: cube1.x1.max(cube2.x1),
                        x2: cube1.x2.min(cube2.x2),
                        y1: cube1.y1.max(cube2.y1),
                        y2: cube1.y2.min(cube2.y2),
                        z1: cube1.z2 + 1,
                        z2: cube2.z2,
                    }));
                }
            }
        }

        if steps[step].0 == "on" {
            cubes.push(Some(cube1.clone()));
        }
    }

    let mut num_cubes_on = 0u128;

    for cube in cubes {
        if let Some(c) = cube {
            let x = (c.x2 - c.x1 + 1) as u128;
            let y = (c.y2 - c.y1 + 1) as u128;
            let z = (c.z2 - c.z1 + 1) as u128;
            let num_cubes = x * y * z;
            num_cubes_on += num_cubes;
        }
    }

    println!("{}", num_cubes_on);
}
