use std::collections::HashMap;

use crate::puzzles::puzzle::Puzzle;
use crate::utils::input_file;

fn count_fish(initial_timers: &Vec<u32>, days: usize) -> u64 {
    // Hint from https://www.reddit.com/r/adventofcode/comments/rdfv7n/comment/ho0sqgs/?utm_source=share&utm_medium=web2x&context=3 to hold a dict of timers
    let initial_histogram: HashMap<_, _> =
        initial_timers.into_iter().fold(HashMap::new(), |mut h, t| {
            h.entry(*t).and_modify(|c| *c += 1).or_insert(1u64);
            h
        });

    let final_histogram = (1..=days).into_iter().fold(initial_histogram, |mut h, _| {
        let new_fish = *h.entry(0).or_insert(0u64);

        for n in 1..=8 {
            let fish_in_n = *h.entry(n).or_insert(0u64);
            h.entry(n).and_modify(|c| *c -= fish_in_n).or_insert(0u64);
            h.entry(n - 1)
                .and_modify(|c| *c += fish_in_n)
                .or_insert(0u64);
        }

        h.entry(0).and_modify(|c| *c -= new_fish).or_insert(0u64);
        h.entry(6).and_modify(|c| *c += new_fish).or_insert(0u64);
        h.entry(8).and_modify(|c| *c += new_fish).or_insert(0u64);

        h
    });

    final_histogram.values().sum()
}

pub struct P6;
impl Puzzle<Vec<u32>> for P6 {
    fn number(&self) -> u8 {
        6
    }

    fn parse_data(&self, raw_data: &Vec<String>) -> Vec<u32> {
        input_file::as_unsigned_ints_from_line(&raw_data[0])
    }

    fn solve_part_one(&self, initial_timers: &Vec<u32>) {
        let count = count_fish(initial_timers, 80);
        println!("{}", count);
    }

    fn solve_part_two(&self, initial_timers: &Vec<u32>) {
        let count = count_fish(initial_timers, 256);
        println!("{}", count);
    }
}
