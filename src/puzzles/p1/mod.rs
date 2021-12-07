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

fn build_list_of_summed_windows(depths: &Vec<u16>) -> Vec<u16> {
    let num_depths = depths.len();
    let mut window_start = 0;
    let mut window_end = 2;
    let mut summed_windows: Vec<u16> = Vec::new();

    for _index in 0..(num_depths - 2) {
        let sum = &depths[window_start..=window_end].iter().sum();
        summed_windows.push(*sum);

        window_start += 1;
        window_end += 1;
    }

    summed_windows
}

pub struct P1;
impl Puzzle<Vec<u16>> for P1 {
    fn number(&self) -> u8 {
        1
    }

    fn parse_data(&self, raw_data: &Vec<String>) -> Vec<u16> {
        raw_data.iter().map(input_file::as_unsigned_short).collect()
    }

    fn solve_part_one(&self, depths: &Vec<u16>) {
        println!("{}", count_increases(depths))
    }

    fn solve_part_two(&self, depths: &Vec<u16>) {
        let summed_windows = build_list_of_summed_windows(depths);
        println!("{}", count_increases(&summed_windows))
    }
}
