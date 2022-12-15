use std::collections::{VecDeque, HashSet, HashMap};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Space {
    Beacon,
    Sensor,
    Empty,
}

fn manhattan_distance(p1: (isize, isize), p2: (isize, isize)) -> usize {
    return ((p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()) as usize;
}

fn find_closers_points(sensor: (isize, isize),
                       beacon: (isize, isize),
                       map: &mut HashMap<(isize, isize), Space>,
                       row: isize) {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let max_dist = manhattan_distance(sensor, beacon);

    if manhattan_distance((sensor.0, row), sensor) < max_dist {
        queue.push_back((sensor.0, row));
        visited.insert((sensor.0, row));
    }

    while queue.len() > 0 {
        let next = queue.pop_front().unwrap();

        for i in [-1, 1] {
            let next_point = (next.0 + i, row);
            let dist = manhattan_distance(next_point, sensor);

            if !visited.contains(&next_point) && dist <= max_dist {
                queue.push_back(next_point);
                visited.insert(next_point);
            }

            if !map.contains_key(&next_point) && dist <= max_dist {
                map.insert(next_point, Space::Empty);
            }
        }
    }
}

fn part1(input: &str, row: isize) -> usize {
    let mut sensors: Vec<(isize, isize)> = Vec::new();
    let mut beacons: Vec<(isize, isize)> = Vec::new();
    let mut map: HashMap<(isize, isize), Space> = HashMap::new();

    for line in input.lines() {
        let (sensor, beacon) = line.split_once(": ").unwrap();

        let sensor = &sensor["Sensor at ".len()..];

        let (x, y) = sensor.split_once(", ").unwrap();

        let x = x[2..].parse::<isize>().unwrap();
        let y = y[2..].parse::<isize>().unwrap();

        sensors.push((x, y));
        map.insert((x, y), Space::Sensor);

        let beacon = &beacon["closest beacon is at ".len()..];
        let (x, y) = beacon.split_once(", ").unwrap();

        let x = x[2..].parse::<isize>().unwrap();
        let y = y[2..].parse::<isize>().unwrap();

        beacons.push((x, y));
        map.insert((x, y), Space::Beacon);
    }

    for i in 0..sensors.len() {
        find_closers_points(sensors[i], beacons[i], &mut map, row);
    }

    return map
        .iter()
        .filter(|((_, y), &s)| *y == row && s != Space::Beacon)
        .count();
}

fn check_hidden(point: (isize, isize), sensors: &Vec<(isize, isize)>, beacons: &Vec<(isize, isize)>, max: usize) -> bool {
    if point.0 > max as isize || point.1 > max as isize  || point.0 < 0 || point.1 < 0 {
        return false;
    }

    for i in 0..sensors.len() {
        if manhattan_distance(sensors[i], beacons[i]) >= manhattan_distance(point, sensors[i]) {
            return false;
        }
    }

    return true;
}

fn part2(input: &str, max: usize) -> usize {
    let mut sensors: Vec<(isize, isize)> = Vec::new();
    let mut beacons: Vec<(isize, isize)> = Vec::new();
    let mut map: HashMap<(isize, isize), Space> = HashMap::new();

    for line in input.lines() {
        let (sensor, beacon) = line.split_once(": ").unwrap();

        let sensor = &sensor["Sensor at ".len()..];

        let (x, y) = sensor.split_once(", ").unwrap();

        let x = x[2..].parse::<isize>().unwrap();
        let y = y[2..].parse::<isize>().unwrap();

        sensors.push((x, y));
        map.insert((x, y), Space::Sensor);

        let beacon = &beacon["closest beacon is at ".len()..];
        let (x, y) = beacon.split_once(", ").unwrap();

        let x = x[2..].parse::<isize>().unwrap();
        let y = y[2..].parse::<isize>().unwrap();

        beacons.push((x, y));
        map.insert((x, y), Space::Beacon);
    }

    for i in 0..sensors.len() {
        let dist = manhattan_distance(sensors[i], beacons[i]) as isize;

        let mut x = sensors[i].0 + dist + 1;
        let mut y = sensors[i].1;

        while x >= sensors[i].0 {
            if check_hidden((x, y), &sensors, &beacons, max) {
                return (x as usize)*4000000 + y as usize;
            }

            x -= 1;
            y += 1;
        }

        let mut x = sensors[i].0 + dist + 1;
        let mut y = sensors[i].1;

        while x >= sensors[i].0 {
            if check_hidden((x, y), &sensors, &beacons, max) {
                return (x as usize)*4000000 + y as usize;
            }

            x -= 1;
            y -= 1;
        }

        let mut x = sensors[i].0 - dist - 1;
        let mut y = sensors[i].1;

        while x <= sensors[i].0 {
            if check_hidden((x, y), &sensors, &beacons, max) {
                return (x as usize)*4000000 + y as usize;
            }

            x += 1;
            y += 1;
        }

        let mut x = sensors[i].0 - dist - 1;
        let mut y = sensors[i].1;

        while x <= sensors[i].0 {
            if check_hidden((x, y), &sensors, &beacons, max) {
                return (x as usize)*4000000 + y as usize;
            }

            x += 1;
            y -= 1;
        }
    }

    unreachable!();
}

fn main() {
    let input = include_str!("./input/day15.input");
    println!("part1: {}", part1(input, 2000000));
    println!("part2: {}", part2(input, 4000000));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "\
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

        assert_eq!(part1(input, 10), 26);
    }

    #[test]
    fn test_part2() {
        let input = "\
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

        assert_eq!(part2(input, 20), 56000011);
    }
}
