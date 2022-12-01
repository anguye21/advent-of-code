use std::{
    collections::{HashMap, LinkedList},
    fs,
};

fn parse_literal_value(transmission: &mut LinkedList<u8>) -> (u64, u64) {
    let mut num_bits_read: u32 = 0;
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

    return (0, (num_bits_read) as u64);
}

fn parse_lid_zero(transmission: &mut LinkedList<u8>) -> (u64, u64) {
    let mut length: i64 = 0;
    let mut version_sum = 0;

    for _ in 0..15 {
        length = (length << 1) | transmission.pop_front().unwrap() as i64;
    }

    let bits_read = length as u64;

    while length > 0 && transmission.len() > 0 {
        let (version, bits_read) = get_next_packet_version_sum(transmission);
        length -= bits_read as i64;
        version_sum += version;
    }

    return (version_sum, bits_read);
}

fn parse_lid_one(transmission: &mut LinkedList<u8>) -> (u64, u64) {
    let mut num_sub_packets: u64 = 0;
    let mut version_sum = 0;
    let mut total_bits_read = 0;

    for _ in 0..11 {
        num_sub_packets = (num_sub_packets << 1) | transmission.pop_front().unwrap() as u64;
    }

    while num_sub_packets > 0 {
        let (version, num_bits_read) = get_next_packet_version_sum(transmission);
        num_sub_packets -= 1;
        version_sum += version;
        total_bits_read += num_bits_read;
    }

    return (version_sum, total_bits_read);
}

fn parse_operator(transmission: &mut LinkedList<u8>) -> (u64, u64) {
    let length_type_id = transmission.pop_front().unwrap();

    if length_type_id == 0 {
        return parse_lid_zero(transmission);
    }

    return parse_lid_one(transmission);
}

fn get_next_packet_version_sum(transmission: &mut LinkedList<u8>) -> (u64, u64) {
    if transmission.iter().fold(0u64, |a, b| a + (*b as u64)) == 0 {
        let ret = (0u64, transmission.len() as u64);
        transmission.clear();
        return ret;
    }

    let version: u64 = ((transmission.pop_front().unwrap() << 2)
        | (transmission.pop_front().unwrap() << 1)
        | (transmission.pop_front().unwrap())) as u64;

    let type_id: u64 = ((transmission.pop_front().unwrap() << 2)
        | (transmission.pop_front().unwrap() << 1)
        | (transmission.pop_front().unwrap())) as u64;

    let (version_sum, num_bits_read) = match type_id {
        4 => parse_literal_value(transmission),
        _ => parse_operator(transmission),
    };

    return (version + version_sum, num_bits_read + 6);
}

fn parse_transmission(transmission: &mut LinkedList<u8>) -> u64 {
    let mut sum: u64 = 0;

    while !transmission.is_empty() {
        if transmission.iter().fold(0u64, |a, b| a + (*b as u64)) == 0 {
            break;
        }
        sum += get_next_packet_version_sum(transmission).0;
    }

    return sum;
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
            let nibble = hex_to_bin[&hex_bit];
            return Vec::from([
                (nibble & 0x8) >> 3,
                (nibble & 0x4) >> 2,
                (nibble & 0x2) >> 1,
                (nibble & 0x1),
            ]);
        })
        .collect();

    println!("{}", parse_transmission(&mut bit_data));
}
