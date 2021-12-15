use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::result::Result::{Err, Ok};

fn open_input_file(num: u8, test_input: bool) -> File {
    let filename_suffix = if test_input { "-test.txt" } else { ".txt" };
    let filename = format!("{}{}", num, filename_suffix);
    let path = Path::new("input-files/").join(Path::new(&filename));

    match File::open(&path) {
        Ok(file) => file,
        Err(_) => {
            let readable_path = path.display();
            panic!(
                "Input file for puzzle {} not found: attempted to read from \"{}\"",
                num, readable_path
            );
        }
    }
}

pub fn read_lines(num: u8, test_input: bool) -> Vec<String> {
    let file = open_input_file(num, test_input);
    BufReader::new(file)
        .lines()
        .map(|line| line.expect("Could not parse line"))
        .collect()
}

// Common parsers here
pub fn as_unsigned_short(item: &String) -> u16 {
    match item.parse::<u16>() {
        Ok(num) => num,
        Err(_) => panic!("Couldn't parse {} as unsigned short", item),
    }
}

pub fn as_chars(item: &String) -> Vec<char> {
    item.chars().collect()
}

pub fn as_contig_unsigned_bytes(line: &String) -> Vec<u8> {
    line.chars().map(|n| n as u8 - 48).collect()
}

pub fn as_unsigned_ints_from_line(line: &String) -> Vec<u32> {
    line.split(',').map(|n| n.parse().unwrap()).collect()
}
