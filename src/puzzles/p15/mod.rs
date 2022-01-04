use lazy_static::lazy_static;

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;

use crate::puzzles::puzzle::Puzzle;
use crate::utils::input_file;

lazy_static! {
    static ref PATH_INCREMENTS: [PathIncr; 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    static ref INFINITY: u32 = u32::MAX;
}

struct Edge {
    risk: u8,
    dest: Index,
}

#[derive(Eq, PartialEq)]
struct Node {
    risk: u32,
    index: Index,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.risk.cmp(&self.risk)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.risk.cmp(&self.risk))
    }
}

type Index = (usize, usize);
type PathIncr = (isize, isize);
type AdjacencyGraph = HashMap<Index, Vec<Edge>>;
type RiskGridData = Vec<Vec<u8>>;

pub struct RiskGrid {
    data: RiskGridData,
    orig_rows: usize,
    orig_cols: usize,
    rows: usize,
    cols: usize
}

impl RiskGrid {
    pub fn new(data: Vec<Vec<u8>>, rows_scale: usize, cols_scale: usize) -> Self {
        let orig_rows = data.len();
        let rows = orig_rows * rows_scale;

        let orig_cols = data[0].len();
        let cols = orig_cols * cols_scale;

        RiskGrid {
            data,
            orig_rows,
            orig_cols,
            rows,
            cols
        }
    }

    pub fn at(&self, (row, col): Index) -> u8 {
        if row < self.orig_rows && col < self.orig_cols {
            self.data[row][col]
        } else {
            let orig_row = row % self.orig_rows;
            let orig_col = col % self.orig_cols;
            let value = self.data[orig_row][orig_col];
            let row_to_add = (row / self.orig_rows) as u8;
            let col_to_add = (col / self.orig_cols) as u8;
            let new_value_wrapped = next_in_range(value, row_to_add + col_to_add, 9);
            new_value_wrapped
        }
    }
}

// Trick to keep numbers in 1..9 range a la https://stackoverflow.com/a/3803420
fn next_in_range(i: u8, m: u8, n: u8) -> u8 {
    (i + m - 1) % n + 1
}

pub struct Graph {
    start: Index,
    end: Index,
    data: AdjacencyGraph,
}
impl Graph {
    pub fn new(grid: &RiskGrid) -> Self {
        let mut adjacency_graph = AdjacencyGraph::new();
        let rows = grid.rows;
        let last_row = rows - 1;
        let cols = grid.cols;
        let last_col = cols - 1;

        for row in 0..rows {
            for col in 0..cols {
                let adjacent_edges = PATH_INCREMENTS
                    .into_iter()
                    .filter_map(|(y_inc, x_inc)| {
                        let invalid_adjacent = y_inc == -1 && row == 0
                            || x_inc == -1 && col == 0
                            || y_inc == 1 && row == last_row
                            || x_inc == 1 && col == last_col;

                        if invalid_adjacent {
                            None
                        } else {
                            let next_row = ((row as isize) + y_inc) as usize;
                            let next_col = ((col as isize) + x_inc) as usize;
                            let edge = Edge {
                                risk: grid.at((next_row, next_col)),
                                dest: (next_row, next_col),
                            };
                            Some(edge)
                        }
                    })
                    .collect::<Vec<_>>();

                let this_index = (row, col);
                adjacency_graph.insert(this_index, adjacent_edges);
            }
        }

        Self {
            start: (0, 0),
            end: (last_row, last_col),
            data: adjacency_graph,
        }
    }
}

/*
   Uses Dijkstra's to compute shortest path.
   This one follows the Rust binary heap example (which enqueues nodes as we go).
   Risks = total risk from start to a given node.
*/
type Risks = HashMap<Index, u32>;
type VisitQueue = BinaryHeap<Node>;
fn compute_size_of_least_risky_path(graph: &Graph) -> u32 {
    let mut risks = Risks::new();
    let mut to_visit = VisitQueue::new();

    // Initial conditions
    risks.insert(graph.start, 0);
    to_visit.push(Node {
        index: graph.start,
        risk: 0,
    });

    while !to_visit.is_empty() {
        if let Some(Node { index, risk }) = to_visit.pop() {
            if index == graph.end {
                return risk;
            }

            let risk_to_current = *risks.get(&index).unwrap_or(&INFINITY);
            if risk > risk_to_current {
                continue;
            }

            if let Some(neighbors) = graph.data.get(&index) {
                for neighbor in neighbors {
                    let Edge {
                        risk: neighbor_risk,
                        dest: neighbor_index,
                    } = neighbor;

                    let risk_to_try = risk + u32::from(*neighbor_risk);
                    let risk_to_neighbor =
                        *risks.get(&neighbor_index).unwrap_or(&INFINITY);
                    let try_neighbor = risk_to_try < risk_to_neighbor;

                    if try_neighbor {
                        risks.insert(*neighbor_index, risk_to_try);

                        to_visit.push(Node {
                            index: *neighbor_index,
                            risk: risk_to_try,
                        });
                    }
                }
            }
        }
    }

    *INFINITY
}

pub struct P15;
impl Puzzle<RiskGridData> for P15 {
    fn number(&self) -> u8 {
        15
    }

    fn parse_data(&self, raw_data: &Vec<String>) -> RiskGridData {
        raw_data
            .iter()
            .map(|line| input_file::as_contig_unsigned_bytes(line))
            .collect::<RiskGridData>()
    }

    fn solve_part_one(&self, grid_data: &RiskGridData) {
        let grid = RiskGrid::new(grid_data.to_vec(), 1, 1);
        let graph = Graph::new(&grid);
        let least_risky_path_size = compute_size_of_least_risky_path(&graph);
        println!("{}", least_risky_path_size);
    }

    fn solve_part_two(&self, grid_data: &RiskGridData) {
        let grid = RiskGrid::new(grid_data.to_vec(), 5, 5);
        let graph = Graph::new(&grid);
        let least_risky_path_size = compute_size_of_least_risky_path(&graph);
        println!("{}", least_risky_path_size);
    }
}
