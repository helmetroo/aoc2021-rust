use std::cmp;

use crate::puzzles::puzzle::Puzzle;
use crate::utils::input_file;

fn build_cost_array(positions: &Vec<u32>) -> Vec<Vec<usize>> {
    // Min is 0, but here for sanity
    let min_pos = positions.iter().min().map(|m| *m as usize).unwrap();
    let max_pos = positions.iter().max().map(|m| *m as usize).unwrap();
    let min_max_diff = max_pos - min_pos;
    let num_pos = positions.len();

    let mut cost_array = vec![vec![0; num_pos]; min_max_diff + 1];
    for row in min_pos..=max_pos {
        for col in 0..num_pos {
            let pos = positions[col] as usize;
            let delta = if pos >= row { pos - row } else { row - pos };

            cost_array[row][col] = delta;
        }
    }
    cost_array
}

fn compute_min_fuel(cost_array: Vec<Vec<usize>>) -> usize {
    cost_array.into_iter().fold(usize::MAX, |cur_min, deltas| {
        cmp::min(cur_min, deltas.iter().sum())
    })
}

pub struct P7;
impl Puzzle<Vec<u32>> for P7 {
    fn number(&self) -> u8 {
        7
    }

    fn parse_data(&self, raw_data: &Vec<String>) -> Vec<u32> {
        input_file::as_unsigned_ints_from_line(&raw_data[0])
    }

    fn solve_part_one(&self, positions: &Vec<u32>) {
        let cost_array = build_cost_array(positions);
        let min_fuel = compute_min_fuel(cost_array);
        println!("{}", min_fuel);
    }

    fn solve_part_two(&self, _positions: &Vec<u32>) {}
}
