use std::{collections::HashMap, fs};

static mut input: Vec<i64> = Vec::new();
static mut input_idx: usize = 0;

fn is_string_numeric(str: String) -> bool {
    for c in str.chars() {
        if !c.is_numeric() && c != '-' {
            return false;
        }
    }
    return true;
}

fn inp(register: &mut i64) {
    unsafe {
        *register = input[input_idx] as i64;
        input_idx += 1;
    }
}

fn add(registers: &mut HashMap<&str, i64>, a: &str, b: &str) {
    let rhs: i64;

    if is_string_numeric(b.to_string()) {
        rhs = b.parse::<i64>().unwrap();
    } else {
        rhs = *registers.get_mut(b).unwrap();
    }

    *registers.get_mut(a).unwrap() += rhs;
}

fn mul(registers: &mut HashMap<&str, i64>, a: &str, b: &str) {
    let rhs: i64;

    if is_string_numeric(b.to_string()) {
        rhs = b.parse::<i64>().unwrap();
    } else {
        rhs = *registers.get_mut(b).unwrap();
    }

    *registers.get_mut(a).unwrap() *= rhs;
}

fn div(registers: &mut HashMap<&str, i64>, a: &str, b: &str) {
    let rhs: i64;

    if is_string_numeric(b.to_string()) {
        rhs = b.parse::<i64>().unwrap();
    } else {
        rhs = *registers.get_mut(b).unwrap();
    }

    *registers.get_mut(a).unwrap() /= rhs;
}

fn modulo(registers: &mut HashMap<&str, i64>, a: &str, b: &str) {
    let rhs: i64;

    if is_string_numeric(b.to_string()) {
        rhs = b.parse::<i64>().unwrap();
    } else {
        rhs = *registers.get_mut(b).unwrap();
    }

    *registers.get_mut(a).unwrap() %= rhs;
}

fn eq(registers: &mut HashMap<&str, i64>, a: &str, b: &str) {
    let rhs: i64;

    if is_string_numeric(b.to_string()) {
        rhs = b.parse::<i64>().unwrap();
    } else {
        rhs = *registers.get(b).unwrap();
    }

    if *registers.get_mut(a).unwrap() == rhs {
        *registers.get_mut(a).unwrap() = 1;
    } else {
        *registers.get_mut(a).unwrap() = 0;
    }
}

fn run_instruction(instruction: &str, registers: &mut HashMap<&str, i64>) {
    let mut i = instruction.splitn(3, ' ');

    let int = i.next().unwrap();
    let lhs = i.next().unwrap();

    if int == "inp" {
        inp(registers.get_mut(lhs).unwrap());
        return;
    }

    let rhs = i.next().unwrap();

    match int {
        "add" => add(registers, lhs, rhs),
        "mul" => mul(registers, lhs, rhs),
        "div" => div(registers, lhs, rhs),
        "mod" => modulo(registers, lhs, rhs),
        "eql" => eq(registers, lhs, rhs),
        _ => panic!(),
    }
}

fn main() {
    let filename = "assets/input-real.txt";

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let program: Vec<&str> = contents.lines().collect();
    let mut registers: HashMap<&str, i64> = HashMap::from([("w", 0), ("x", 0), ("y", 0), ("z", 0)]);

    unsafe {
        input = fs::read_to_string("assets/input.txt")
            .unwrap()
            .lines()
            .map(|line| line.parse().unwrap())
            .collect();
    }

    for line in &program {
        run_instruction(line, &mut registers);
    }

    println!("w = {}", registers["w"]);
    println!("x = {}", registers["x"]);
    println!("y = {}", registers["y"]);
    println!("z = {}", registers["z"]);
}
