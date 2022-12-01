use std::{
    collections::{HashMap, HashSet},
    fs, ops,
    str::FromStr,
};

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Position(i32, i32, i32);

impl FromStr for Position {
    type Err = std::string::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut x = s.splitn(3, ',');

        return Ok(Position(
            x.next().unwrap().parse::<i32>().unwrap(),
            x.next().unwrap().parse::<i32>().unwrap(),
            x.next().unwrap().parse::<i32>().unwrap(),
        ));
    }
}

impl ops::Sub<Position> for Position {
    type Output = Position;

    fn sub(self, rhs: Position) -> Self::Output {
        return Position(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2);
    }
}

impl ops::Add<Position> for Position {
    type Output = Position;

    fn add(self, rhs: Position) -> Self::Output {
        return Position(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2);
    }
}

fn get_distances(scanner: &Vec<Position>) -> Vec<Vec<HashSet<u32>>> {
    let mut distance_sets: Vec<Vec<HashSet<u32>>> = Vec::new();

    for i in 0..scanner.len() {
        distance_sets.push(Vec::new());
        for j in 0..scanner.len() {
            let abs_pos_i = Position(scanner[i].0, scanner[i].1, scanner[i].2);
            let abs_pos_j = Position(scanner[j].0, scanner[j].1, scanner[j].2);

            let dist_set = HashSet::from([
                (abs_pos_i.0 - abs_pos_j.0).abs() as u32,
                (abs_pos_i.1 - abs_pos_j.1).abs() as u32,
                (abs_pos_i.2 - abs_pos_j.2).abs() as u32,
            ]);

            distance_sets[i].push(dist_set);
        }
    }

    return distance_sets;
}

fn find_dimension_and_sign(diff_1: Position, diff_2: Position) -> ([usize; 3], [i32; 3]) {
    let mut dimension: [usize; 3] = [0, 1, 2];
    let mut sign: [i32; 3] = [1, 1, 1];

    for (i, val1) in [diff_1.0, diff_1.1, diff_1.2].iter().enumerate() {
        for (j, val2) in [diff_2.0, diff_2.1, diff_2.2].iter().enumerate() {
            if val1.abs() == val2.abs() {
                dimension[i] = j;
                sign[i] = val1 / val2;
                break;
            }
        }
    }

    return (dimension, sign);
}

fn check_overlap(
    s1: &Vec<Position>,
    s2: &Vec<Position>,
    distance_set_1: &Vec<Vec<HashSet<u32>>>,
    distance_set_2: &Vec<Vec<HashSet<u32>>>,
) -> (bool, [usize; 3], [i32; 3], Position) {
    let mut overlap: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();
    let mut dimension: [usize; 3] = [0, 1, 2];
    let mut sign: [i32; 3] = [1, 1, 1];
    let mut s2_abs_pos = Position(0, 0, 0);

    for (i, distances_1) in distance_set_1.iter().enumerate() {
        for (j, distances_2) in distance_set_2.iter().enumerate() {
            let mut count = 0;
            let mut same_dist_indicies: Vec<(usize, usize)> = Vec::new();

            for (i_dist, dist_1) in distances_1.iter().enumerate() {
                if *dist_1 == HashSet::from([0]) {
                    continue;
                }

                for (j_dist, dist_2) in distances_2.iter().enumerate() {
                    if dist_1 == dist_2 {
                        count += 1;
                        same_dist_indicies.push((i_dist, j_dist));
                        break;
                    }
                }
            }

            if count >= 11 {
                overlap.insert((i, j), same_dist_indicies);
            }
        }
    }

    let mut i_diff = Position(1, 1, 1);
    let mut j_diff = Position(1, 1, 1);

    if overlap.len() >= 11 {
        for ((i1, j1), ij) in &overlap {
            i_diff = s1[*i1] - s1[ij[0].0];
            j_diff = s2[*j1] - s2[ij[0].1];

            if i_diff.0.abs() != i_diff.1.abs() && i_diff.0.abs() != i_diff.2.abs() {
                break;
            }
        }

        let (dimension_tmp, sign_tmp) = find_dimension_and_sign(i_diff, j_diff);
        dimension = dimension_tmp;
        sign = sign_tmp;
        let mut overlap_iter = overlap.iter();
        let ((i, j), _) = overlap_iter.next().unwrap();
        let tmp = [s2[*j].0, s2[*j].1, s2[*j].2];
        s2_abs_pos = Position(
            sign[0] * tmp[dimension[0]],
            sign[1] * tmp[dimension[1]],
            sign[2] * tmp[dimension[2]],
        );
        s2_abs_pos = s1[*i] - s2_abs_pos;
    }

    return (overlap.len() >= 11, dimension, sign, s2_abs_pos);
}

// https://libreddit.kavin.rocks/r/adventofcode/comments/rjwkuf/day_19_how_to_calculate_the_scanner_position/
fn num_beacons(scanners: &mut Vec<Vec<Position>>) -> usize {
    let mut found_abs = Vec::from([0]);
    let mut beacons = HashSet::new();

    let mut distance_sets: Vec<Vec<Vec<HashSet<u32>>>> = Vec::new();

    for s in 0..scanners.len() {
        distance_sets.push(get_distances(&scanners[s]));
    }

    //for i in 0..scanners.len() {
    let mut idx = 0;
    while found_abs.len() < scanners.len() {
        let i = found_abs[idx];
        for j in 0..scanners.len() {
            if found_abs.contains(&j) || i == j {
                continue;
            }

            let (overlap, dimension, sign, j_abs_pos) = check_overlap(
                &scanners[i],
                &scanners[j],
                &distance_sets[i],
                &distance_sets[j],
            );

            if overlap {
                for idx in 0..scanners[j].len() {
                    let pos = scanners[j][idx];
                    let tmp = [pos.0, pos.1, pos.2];
                    scanners[j][idx] = Position(
                        sign[0] * tmp[dimension[0]],
                        sign[1] * tmp[dimension[1]],
                        sign[2] * tmp[dimension[2]],
                    ) + j_abs_pos;
                }
                found_abs.push(j);
            }
        }

        idx += 1;
    }

    for scanner in scanners {
        for beacon_pos in scanner {
            beacons.insert(beacon_pos);
        }
    }

    return beacons.len();
}

fn main() {
    let filename = "assets/input-real.txt";

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let data: Vec<&str> = contents.lines().skip(1).collect();

    let mut scanners: Vec<Vec<Position>> = Vec::new();
    let mut curr_scanner: usize = 0;

    scanners.push(Vec::new());

    for line in data {
        if line.starts_with("---") {
            curr_scanner += 1;
            scanners.push(Vec::new());
            continue;
        }

        if line == "" {
            continue;
        }

        scanners[curr_scanner].push(line.parse().unwrap());
    }

    println!("{}", num_beacons(&mut scanners));
}
