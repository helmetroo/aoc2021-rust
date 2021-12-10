use std::iter;

use crate::puzzles::puzzle::Puzzle;

#[warn(unused_assignments)]
fn count_fish(initial_timers: &Vec<u32>, iterations: u32) -> usize {
    let mut final_timers = initial_timers.clone();
    for _ in 0..iterations {
        let mut zeroes_counted = 0;
        for timer in final_timers.iter_mut() {
            let zero = &mut 0;
            if timer > zero {
                *timer -= 1;
            } else {
                zeroes_counted += 1;
                *timer = 6;
            }
        }

        final_timers.extend(
            iter::repeat(8u32)
                .take(zeroes_counted)
                .collect::<Vec<u32>>(),
        );
        zeroes_counted = 0;
    }

    final_timers.len()
}

pub struct P6;
impl Puzzle<Vec<u32>> for P6 {
    fn number(&self) -> u8 {
        6
    }

    fn parse_data(&self, raw_data: &Vec<String>) -> Vec<u32> {
        let list = &raw_data[0];
        list.split(',').map(|n| n.parse().unwrap()).collect()
    }

    fn solve_part_one(&self, initial_timers: &Vec<u32>) {
        let count = count_fish(initial_timers, 80);
        println!("{}", count);
    }

    fn solve_part_two(&self, _timers: &Vec<u32>) {}
}
