use std::collections::HashSet;

use crate::puzzles::puzzle::Puzzle;
use crate::utils::input_file;

type HeightMap = Vec<Vec<u8>>;
type Position = (usize, usize);
type PathIncr = (isize, isize);
struct Edge {
    from: Position,
    to: Position,
}
type AdjacencyList = Vec<Edge>;
type PosSet = HashSet<Position>;

fn build_adjacency_list(height_map: &HeightMap) -> AdjacencyList {
    let mut adjacency_list = AdjacencyList::new();
    let rows = height_map.len();
    let cols = height_map[0].len();

    for row in 0..rows {
        for col in 0..cols {
            let current = height_map[row][col];
            let path_increments: Vec<PathIncr> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
            let adjacent_edges = path_increments
                .into_iter()
                .map(|(y_inc, x_inc)| {
                    // Pushes us out of height map bounds
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
                // Keep only valid positions
                .filter_map(|adj_pos| adj_pos)
                .filter(|&(row, col)| {
                    let adjacent = height_map[row][col];
                    adjacent <= current
                })
                .map(|adjacent| Edge {
                    from: (row, col),
                    to: adjacent,
                });

            adjacency_list.extend(adjacent_edges);
        }
    }
    adjacency_list
}

fn build_positions_set(height_map: &HeightMap) -> PosSet {
    let rows = height_map.len();
    let cols = height_map[0].len();

    let mut indices = PosSet::new();
    for row in 0..rows {
        for col in 0..cols {
            indices.insert((row, col));
        }
    }
    indices
}

fn find_sink_positions(height_map: &HeightMap, adjacency_list: &AdjacencyList) -> Vec<Position> {
    let all_positions = build_positions_set(height_map);
    let visited = PosSet::from_iter(adjacency_list.iter().map(|edge| edge.from));
    all_positions.difference(&visited).map(|&pos| pos).collect()
}

fn sum_risk_levels(height_map: &HeightMap) -> u32 {
    let adjacency_list = build_adjacency_list(height_map);
    let sinks = find_sink_positions(height_map, &adjacency_list);
    sinks
        .iter()
        .map(|&(row, col)| (height_map[row][col] + 1) as u32)
        .sum()
}

pub struct P9;
impl Puzzle<HeightMap> for P9 {
    fn number(&self) -> u8 {
        9
    }

    fn parse_data(&self, raw_data: &Vec<String>) -> HeightMap {
        raw_data
            .iter()
            .map(|line| input_file::as_contig_unsigned_bytes(line))
            .collect::<HeightMap>()
    }

    fn solve_part_one(&self, height_map: &HeightMap) {
        let sum_risk_levels = sum_risk_levels(height_map);
        println!("{}", sum_risk_levels);
    }

    fn solve_part_two(&self, _height_map: &HeightMap) {}
}
