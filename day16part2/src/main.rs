use std::{
    collections::{HashMap, LinkedList},
    fs,
};

static mut indent: u32 = 0;

fn extact_literal(transmission: &mut LinkedList<u8>) -> (u64, u64) {
    let mut num_bits_read: u64 = 0;
    let mut literal: u64 = 0;

    loop {
        let last_group = transmission.pop_front().unwrap() == 0;

        for _ in 0..4 {
            literal = literal << 1 | transmission.pop_front().unwrap() as u64;
        }

        num_bits_read += 5;

        if last_group {
            break;
        }
    }

    return (literal, num_bits_read);
}

fn evaluate_lid_zero(
    transmission: &mut LinkedList<u8>,
    op: &dyn Fn(u64, u64) -> u64,
) -> (u64, u64) {
    let mut length: u64 = 0;
    let mut result: u64;
    let mut values: Vec<u64> = Vec::new();
    let mut sub_packet: LinkedList<u8> = LinkedList::new();

    for _ in 0..15 {
        length = (length << 1) | transmission.pop_front().unwrap() as u64;
    }

    for _ in 0..length {
        sub_packet.push_back(transmission.pop_front().unwrap());
    }

    while !sub_packet.is_empty() {
        let (expr_result, bits_read) = evaluate_packet(&mut sub_packet);

        if bits_read > 0 {
            values.push(expr_result);
        }
    }

    unsafe {
        for _ in 0..indent {
            print!(" ");
        }
    }

    println!("{:?}", values);
    result = values[0];
    for value in 1..values.len() {
        result = op(result, values[value]);
    }

    return (result, length);
}

fn evaluate_lid_one(transmission: &mut LinkedList<u8>, op: &dyn Fn(u64, u64) -> u64) -> (u64, u64) {
    let mut num_sub_packets: u64 = 0;
    let mut total_bits_read = 0;
    let mut result: u64;
    let mut values: Vec<u64> = Vec::new();

    for _ in 0..11 {
        num_sub_packets = (num_sub_packets << 1) | transmission.pop_front().unwrap() as u64;
    }

    while num_sub_packets > 0 {
        let (expr_result, num_bits_read) = evaluate_packet(transmission);
        num_sub_packets -= 1;
        total_bits_read += num_bits_read;

        if num_bits_read > 0 {
            values.push(expr_result);
        }
    }

    unsafe {
        for _ in 0..indent {
            print!(" ");
        }
    }

    println!("{:?}", values);
    result = values[0];
    for value in 1..values.len() {
        result = op(result, values[value]);
    }

    return (result, total_bits_read);
}

fn evaluate_operator(
    transmission: &mut LinkedList<u8>,
    op: &dyn Fn(u64, u64) -> u64,
) -> (u64, u64) {
    let length_type_id = transmission.pop_front().unwrap();

    if length_type_id == 0 {
        return evaluate_lid_zero(transmission, op);
    }

    return evaluate_lid_one(transmission, op);
}

fn evaluate_packet(transmission: &mut LinkedList<u8>) -> (u64, u64) {
    if transmission.iter().fold(0u64, |a, b| a + (*b as u64)) == 0 {
        transmission.clear();
        return (0, 0);
    }

    let _version: u64 = ((transmission.pop_front().unwrap() << 2)
        | (transmission.pop_front().unwrap() << 1)
        | (transmission.pop_front().unwrap())) as u64;

    let type_id: u64 = ((transmission.pop_front().unwrap() << 2)
        | (transmission.pop_front().unwrap() << 1)
        | (transmission.pop_front().unwrap())) as u64;

    unsafe {
        for _ in 0..indent {
            print!(" ");
        }
    }

    println!(
        "op: {} {{",
        match type_id {
            0 => "sum",
            1 => "product",
            2 => "minimum",
            3 => "maximum",
            4 => "literal",
            5 => "greater than",
            6 => "less than",
            7 => "equal to",
            _ => unreachable!(),
        }
    );

    unsafe {
        indent += 2;
    }

    let (result, num_bits_read) = match type_id {
        0 => evaluate_operator(transmission, &|a, b| a + b),
        1 => evaluate_operator(transmission, &|a, b| a * b),
        2 => evaluate_operator(transmission, &|a, b| if a < b { a } else { b }),
        3 => evaluate_operator(transmission, &|a, b| if a > b { a } else { b }),
        4 => extact_literal(transmission),
        5 => evaluate_operator(transmission, &|a, b| if a > b { 1 } else { 0 }),
        6 => evaluate_operator(transmission, &|a, b| if a < b { 1 } else { 0 }),
        7 => evaluate_operator(transmission, &|a, b| if a == b { 1 } else { 0 }),
        _ => unreachable!(),
    };

    unsafe {
        for _ in 0..indent {
            print!(" ");
        }
    }

    println!("result: {}", result);

    unsafe {
        indent -= 2;
    }

    unsafe {
        for _ in 0..indent {
            print!(" ");
        }
    }

    println!("}}\n");

    return (result, num_bits_read + 6);
}

fn main() {
    let filename = "assets/input-real.txt";

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let data: Vec<&str> = contents.lines().collect();

    let hex_to_bin: HashMap<char, u8> = HashMap::from([
        ('0', 0),
        ('1', 1),
        ('2', 2),
        ('3', 3),
        ('4', 4),
        ('5', 5),
        ('6', 6),
        ('7', 7),
        ('8', 8),
        ('9', 9),
        ('A', 10),
        ('B', 11),
        ('C', 12),
        ('D', 13),
        ('E', 14),
        ('F', 15),
    ]);

    let mut bit_data: LinkedList<u8> = data[0]
        .chars()
        .flat_map(|hex_bit| {
            let nibble = hex_to_bin[&(hex_bit.to_ascii_uppercase())];
            return Vec::from([
                (nibble & 0x8) >> 3,
                (nibble & 0x4) >> 2,
                (nibble & 0x2) >> 1,
                (nibble & 0x1),
            ]);
        })
        .collect();

    println!("{}", evaluate_packet(&mut bit_data).0);
}
