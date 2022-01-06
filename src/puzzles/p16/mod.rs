use crate::puzzles::puzzle::Puzzle;

// Version can fit into u8
#[derive(Debug)]
pub enum Packet {
    Literal {
        version: u64,
        value: u64,
    },
    Operator {
        version: u64,
        subpackets: Vec<Packet>,
    },
}

fn char_to_binary_string(ch: char) -> String {
    let value = match ch {
        'A'..='F' => (ch as u8) - ('A' as u8) + 0xA,
        _ => (ch as u8) - ('0' as u8),
    };

    // Using # adds 0b prefix, which we don't want, and we also need a 4-chr string
    format!("{:#06b}", value)[2..].to_string()
}

fn parse_message(message: &String) -> Packet {
    let message_bin_str = message
        .chars()
        .map(char_to_binary_string)
        .collect::<Vec<_>>()
        .join("");

    parse_binary_string(&message_bin_str).0
}

fn parse_binary_string(bin_str: &str) -> (Packet, usize) {
    let mut cursor = 0usize;

    let (version, version_bits) = read_bits(&bin_str[cursor..], 3);
    cursor += version_bits;

    let (id, id_bits) = read_bits(&bin_str[cursor..], 3);
    cursor += id_bits;

    let literal = id == 4;
    if literal {
        let (value, literal_bits) = read_literal_value(&bin_str[cursor..]);
        cursor += literal_bits;

        (Packet::Literal { version, value }, cursor)
    } else {
        let (is_num_subpackets, length_bits) = read_bool(&bin_str[cursor..]);
        cursor += length_bits;

        let mut subpackets = Vec::new();
        if is_num_subpackets {
            let (num_subpackets, length_bits) = read_bits(&bin_str[cursor..], 11);
            cursor += length_bits;

            for _ in 0..num_subpackets {
                let (subpacket, subpacket_bits) = parse_binary_string(&bin_str[cursor..]);
                cursor += subpacket_bits;

                subpackets.push(subpacket);
            }
        } else {
            // Otherwise it's total number of bits the subpackets occupy
            let (total_subpacket_bits, length_bits) = read_bits(&bin_str[cursor..], 15);
            cursor += length_bits;

            let mut subpacket_bits_read = 0usize;
            while subpacket_bits_read < total_subpacket_bits.try_into().unwrap() {
                let (subpacket, subpacket_bits) = parse_binary_string(&bin_str[cursor..]);
                cursor += subpacket_bits;
                subpacket_bits_read += subpacket_bits;

                subpackets.push(subpacket);
            }
        }

        (
            Packet::Operator {
                version,
                subpackets,
            },
            cursor,
        )
    }
}

fn read_bits(bin_str: &str, size: usize) -> (u64, usize) {
    let mut length = size - 1;
    let mut value = 0u64;

    for bit in bin_str.chars() {
        let bit_value = if bit == '1' { 1 } else { 0 };
        value |= bit_value << length;

        if length == 0 {
            break;
        }
        length -= 1;
    }

    let bin_str_size = bin_str.len();
    let bits_read = if bin_str_size <= size {
        bin_str_size
    } else {
        size
    };
    (value, bits_read)
}

fn read_bool(bin_str: &str) -> (bool, usize) {
    let value = bin_str.starts_with('1');
    (value, 1)
}

fn read_literal_value(bin_str: &str) -> (u64, usize) {
    let mut keep_reading = true;
    let mut bits_counted = 0usize;
    let mut literal_bin_str = String::new();
    while keep_reading {
        let cur_group = &bin_str[bits_counted..bits_counted + 5];
        let continue_bit = &cur_group[0..1];
        let (will_read_next, _) = read_bool(continue_bit);
        bits_counted += 1;
        keep_reading = will_read_next;

        let group_bin_str = &cur_group[1..];
        literal_bin_str += group_bin_str;
        bits_counted += 4;
    }

    let (value, _) = read_bits(&literal_bin_str[..], literal_bin_str.len());
    (value, bits_counted)
}

fn sum_versions(packet: &Packet) -> u64 {
    match packet {
        Packet::Operator {
            version,
            subpackets,
        } => subpackets
            .iter()
            .fold(*version, |sum, packet| sum + sum_versions(packet)),
        Packet::Literal { version, .. } => *version,
    }
}

pub struct P16;
impl Puzzle<Packet> for P16 {
    fn number(&self) -> u8 {
        16
    }

    fn parse_data(&self, raw_data: &Vec<String>) -> Packet {
        let message = &raw_data[0];
        parse_message(message)
    }

    fn solve_part_one(&self, packet: &Packet) {
        let version_sum = sum_versions(packet);
        println!("{}", version_sum);
    }

    fn solve_part_two(&self, _packet: &Packet) {}
}
