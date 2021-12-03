use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::result::Result::{Err, Ok};

pub fn read(num: u8) -> Vec<String> {
    let filename = format!("{}.txt", num);
    let path = Path::new("input-files/").join(Path::new(&filename));

    let file = match File::open(&path) {
        Ok(file) => file,
        Err(_) => {
            let readable_path = path.display();
            panic!(
                "Input file for puzzle {} not found: attempted to read from \"{}\"",
                num, readable_path
            );
        }
    };

    BufReader::new(file)
        .lines()
        .map(|line| line.expect("Could not parse line"))
        .collect()
}

pub fn as_unsigned_short(item: &String) -> u16 {
    match item.parse::<u16>() {
        Ok(num) => num,
        Err(_) => panic!("Couldn't parse {} as unsigned short", item),
    }
}
