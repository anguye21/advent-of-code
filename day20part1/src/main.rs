use std::{collections::HashMap, fs};

fn main() {
    let filename = "assets/input-real.txt";

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let data: Vec<&str> = contents.lines().collect();

    let image_enhancement_algorithm = data[0];

    let input_image: Vec<Vec<char>> = data
        .iter()
        .skip(2)
        .map(|row| row.chars().collect())
        .collect();

    let mut image: HashMap<(i32, i32), char> = HashMap::new();
    let mut row_max = input_image.len() as i32;
    let mut col_max = input_image[0].len() as i32;
    let mut row_min = 0;
    let mut col_min = 0;
    let mut infinity = '.';

    for (i, row) in input_image.iter().enumerate() {
        for (j, pixel) in row.iter().enumerate() {
            image.insert((i as i32, j as i32), *pixel);
        }
    }

    for _ in 0..2 {
        let prev_image = image.clone();

        for row in (row_min - 1)..=(row_max + 1) {
            for col in (col_min - 1)..=(col_max + 1) {
                let mut index = 0;

                for i in (row - 1)..=(row + 1) {
                    for j in (col - 1)..=(col + 1) {
                        index <<= 1;

                        let pixel = if prev_image.contains_key(&(i, j)) {
                            *prev_image.get(&(i, j)).unwrap()
                        } else {
                            infinity
                        };

                        if pixel == '#' {
                            index |= 1;
                        }
                    }
                }

                *image.entry((row, col)).or_insert('.') =
                    image_enhancement_algorithm.chars().nth(index).unwrap();
            }
        }

        row_min -= 1;
        col_min -= 1;
        row_max += 1;
        col_max += 1;

        if image_enhancement_algorithm.chars().nth(0).unwrap()
            != image_enhancement_algorithm.chars().last().unwrap()
        {
            if infinity == '.' {
                infinity = '#'
            } else {
                infinity = '.'
            }
        }
    }

    println!(
        "{}",
        image
            .iter()
            .fold(0, |accum, ((_, _), pixel)| if *pixel == '#' {
                accum + 1
            } else {
                accum
            })
    );
}
