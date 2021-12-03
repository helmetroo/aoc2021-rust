use crate::puzzles::puzzle::Puzzle;
use crate::utils::input_file;

fn count_increases(depths: &Vec<u16>) -> u16 {
    let num_depths = depths.len();
    let mut increases = 0u16;

    for index in 0..(num_depths - 1) {
        let next_index = index + 1;

        let current = depths[index];
        let next = depths[next_index];

        if next > current {
            increases += 1;
        }
    }

    increases
}

pub struct P1;
impl Puzzle<u16> for P1 {
    fn number(&self) -> u8 {
        1
    }

    fn parse_data(&self, raw_data: &Vec<String>) -> Vec<u16> {
        raw_data.iter().map(input_file::as_unsigned_short).collect()
    }

    fn solve_part_one(&self, depths: &Vec<u16>) {
        println!("{}", count_increases(depths))
    }

    fn solve_part_two(&self, depths: &Vec<u16>) {}
}
