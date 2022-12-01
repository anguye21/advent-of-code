use std::fs;

struct Point {
    x: i32,
    y: i32,
}

struct LineSegment {
    start: Point,
    end: Point,
}

macro_rules! max {
    ($x:expr) => ( $x );
    ($x:expr, $($xs:expr),+) => {
        {
            use std::cmp::max;
            max($x, max!( $($xs),+ ))
        }
    };
}

macro_rules! min {
    ($x:expr) => ( $x );
    ($x:expr, $($xs:expr),+) => {
        {
            use std::cmp::min;
            min($x, min!( $($xs),+ ))
        }
    };
}

fn main() {
    let filename = "assets/input-real.txt";

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let data: Vec<&str> = contents.lines().collect();

    let mut num_overlaps: i32 = 0;

    let mut line_segments: Vec<LineSegment> = Vec::new();
    let mut board: Vec<Vec<i32>>;
    let mut x_max: usize = 0;
    let mut y_max: usize = 0;

    for line in data {
        let points: Vec<&str> = line.split(" -> ").collect();
        let p1: Vec<&str> = points[0].split(",").collect();
        let p2: Vec<&str> = points[1].split(",").collect();

        let x1: i32 = p1[0].parse().unwrap();
        let y1: i32 = p1[1].parse().unwrap();
        let x2: i32 = p2[0].parse().unwrap();
        let y2: i32 = p2[1].parse().unwrap();

        let line_segment = LineSegment {
            start: Point { x: x1, y: y1 },
            end: Point { x: x2, y: y2 },
        };

        line_segments.push(line_segment);

        x_max = max!(x_max, x1 as usize);
        x_max = max!(x_max, x2 as usize);

        y_max = max!(y_max, y1 as usize);
        y_max = max!(y_max, y2 as usize);
    }

    board = vec![vec![0; x_max + 1]; y_max + 1];

    for line_segment in line_segments {
        let x1: i32 = line_segment.start.x;
        let y1: i32 = line_segment.start.y;
        let x2: i32 = line_segment.end.x;
        let y2: i32 = line_segment.end.y;

        if x1 == x2 {
            let y_min = min!(y1, y2);
            let y_max = max!(y1, y2);

            for y in y_min..(y_max + 1) {
                board[y as usize][x1 as usize] += 1;
            }
        } else if y1 == y2 {
            let x_min = min!(x1, x2);
            let x_max = max!(x1, x2);

            for x in x_min..(x_max + 1) {
                board[y1 as usize][x as usize] += 1;
            }
        } else {
            let x_min = min!(x1, x2);
            let x_max = max!(x1, x2);

            let y_min = min!(y1, y2);
            let y_max = max!(y1, y2);

            let slope: i32 = (y2 - y1) / (x2 - x1);
            let mut y_start: i32 = y_min;

            if slope == -1 {
                y_start = y_max;
            }

            for step in 0..(x_max - x_min + 1) {
                board[(y_start + slope * step) as usize][(x_min + step) as usize] += 1;
            }
        }
    }

    for line in board {
        for val in line {
            if val > 1 {
                num_overlaps += 1;
            }
        }
    }

    println!("{}", num_overlaps);
}
