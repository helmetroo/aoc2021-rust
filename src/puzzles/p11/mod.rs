use std::collections::HashSet;

use crate::puzzles::puzzle::Puzzle;
use crate::utils::input_file;

type EnergyMap = Vec<Vec<u8>>;
type Position = (usize, usize);
type PathIncr = (isize, isize);
type CellSet = HashSet<Position>;
type AdjacentCells = Vec<Position>;

fn count_flashes(energy_map: &EnergyMap, steps: usize) -> u32 {
    let rows = energy_map.len();
    let cols = energy_map[0].len();
    let mut flashes = 0;
    let mut final_map = energy_map.clone();

    for _step in 0..steps {
        let mut flashed_cells = CellSet::new();

        // Increment energies
        for row in 0..rows {
            for col in 0..cols {
                let cell = &mut final_map[row][col];
                *cell += 1;

                if *cell > 9 {
                    flashed_cells.insert((row, col));
                }
            }
        }

        // Increment energies of those adjacent to those that first flashed
        let mut adjacents = get_adjacents(&flashed_cells, rows, cols);
        while !adjacents.is_empty() {
            let mut all_new_adjacents = AdjacentCells::new();
            for &pos in adjacents.iter() {
                if flashed_cells.contains(&pos) {
                    continue;
                }

                let (row, col) = pos;
                let cell = &mut final_map[row][col];
                *cell += 1;

                if *cell > 9 {
                    flashed_cells.insert(pos);
                    let new_adjacents = get_adjacents_from(pos, rows, cols);
                    all_new_adjacents.extend(new_adjacents);
                }

            }

            adjacents.clear();
            adjacents.extend(all_new_adjacents);
        }

        // Reset and count flashed
        for (row, col) in flashed_cells {
            let cell = &mut final_map[row][col];
            flashes += 1;
            *cell = 0;
        }
    }

    flashes
}

fn get_adjacents(cells: &CellSet, rows: usize, cols: usize) -> AdjacentCells {
    let mut all_adjacents = AdjacentCells::new();
    for &pos in cells {
        let adjacents = get_adjacents_from(pos, rows, cols);
        all_adjacents.extend(adjacents);
    }

    all_adjacents
}

fn get_adjacents_from((row, col): Position, rows: usize, cols: usize) -> Vec<Position> {
    let path_increments: Vec<PathIncr> = vec![
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
        (-1, -1),
        (-1, 1),
        (1, -1),
        (1, 1),
    ];

    path_increments
        .into_iter()
        .map(|(y_inc, x_inc)| {
            // Pushes us out of bounds?
            if y_inc == -1 && row == 0
                || x_inc == -1 && col == 0
                || y_inc == 1 && row == rows - 1
                || x_inc == 1 && col == cols - 1
            {
                None
            } else {
                let next_row = ((row as isize) + y_inc) as usize;
                let next_col = ((col as isize) + x_inc) as usize;
                Some((next_row, next_col))
            }
        })
        .filter_map(|pos| pos)
        .collect()
}

pub struct P11;
impl Puzzle<EnergyMap> for P11 {
    fn number(&self) -> u8 {
        11
    }

    fn parse_data(&self, raw_data: &Vec<String>) -> EnergyMap {
        raw_data
            .iter()
            .map(|line| input_file::as_contig_unsigned_bytes(line))
            .collect::<EnergyMap>()
    }

    fn solve_part_one(&self, energy_map: &EnergyMap) {
        let flashes = count_flashes(energy_map, 100);
        println!("{}", flashes);
    }

    fn solve_part_two(&self, _energy_map: &EnergyMap) {}
}
