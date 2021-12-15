use std::{collections::HashMap, fs};

fn shortest_path_risk(map: &Vec<Vec<u32>>) -> u32 {
    let (rows, cols) = (map.len(), map[0].len());

    let delta = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut risk: HashMap<(i32, i32), u32> = HashMap::new();
    let mut queue: Vec<(u32, (usize, usize))> = Vec::from([(0, (0, 0))]);

    for i in 0..rows {
        for j in 0..cols {
            risk.insert((i as i32, j as i32), std::u32::MAX);
        }
    }

    while !queue.is_empty() {
        queue.sort_by(|a, b| b.0.cmp(&a.0));
        let (curr_risk, pos) = queue.pop().unwrap();

        delta.iter().for_each(|(di, dj)| {
            let ni = (pos.0 as i32) + di;
            let nj = (pos.1 as i32) + dj;

            if risk.contains_key(&(ni, nj)) {
                let new_risk = curr_risk + map[ni as usize][nj as usize];

                if new_risk < risk[&(ni, nj)] {
                    *risk.get_mut(&(ni, nj)).unwrap() = new_risk;
                    queue.push((new_risk, (ni as usize, nj as usize)));
                }
            }
        });
    }

    return risk[&(rows as i32 - 1, cols as i32 - 1)];
}

fn main() {
    let filename = "assets/input-real.txt";

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let data: Vec<&str> = contents.lines().collect();

    let map: Vec<Vec<u32>> = data
        .into_iter()
        .map(|row| {
            return row
                .chars()
                .map(|risk_level| risk_level.to_digit(10).unwrap())
                .collect();
        })
        .collect();

    println!("{}", shortest_path_risk(&map));
}
