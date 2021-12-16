use std::collections::HashMap;
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
type AdjacencyGraph = HashMap<Position, Vec<Position>>;
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

fn build_inv_adjacency_graph(adjacency_list: &AdjacencyList, height_map: &HeightMap) -> AdjacencyGraph {
    adjacency_list
        .into_iter()
        .fold(AdjacencyGraph::new(), |mut graph, edge| {
            let Edge { from, to } = edge;
            let low_enough = height_map[from.0][from.1] < 9;
            if low_enough {
                // Must be indirected (hence the redundant to -> from, from -> to)
                let adjacents = graph.entry(*to).or_insert(Vec::new());
                adjacents.push(*from);

                let adjacents = graph.entry(*from).or_insert(Vec::new());
                adjacents.push(*to);
            }

            graph
        })
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

fn find_basins(sinks: &Vec<Position>, adjacency_graph: &AdjacencyGraph) -> Vec<PosSet> {
    sinks
        .iter()
        .map(|sink| compute_basin(sink, adjacency_graph))
        .collect()
}

// Depth-first search to get all the nodes from the sink
fn compute_basin(sink: &Position, adjacency_graph: &AdjacencyGraph) -> PosSet {
    let mut basin = PosSet::new();
    let mut to_visit = adjacency_graph[sink].clone();
    while !to_visit.is_empty() {
        to_visit.pop().and_then(|next| {
            let unvisited = !basin.contains(&next);
            if unvisited {
                basin.insert(next);
                if adjacency_graph.contains_key(&next) {
                    to_visit.extend(adjacency_graph[&next].clone());
                }
            }

            Some(())
        });
    }

    basin
}

fn mul_largest_three_basins(height_map: &HeightMap) -> u32 {
    let adjacency_list = build_adjacency_list(height_map);
    let adjacency_graph = build_inv_adjacency_graph(&adjacency_list, &height_map);
    let sinks = find_sink_positions(height_map, &adjacency_list);
    let mut basins = find_basins(&sinks, &adjacency_graph);

    // Descending order by num nodes
    basins.sort_by(|a, b| b.len().cmp(&a.len()));

    basins[0..3]
        .iter()
        .fold(1u32, |acc, basin| acc * (basin.len() as u32))
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

    fn solve_part_two(&self, height_map: &HeightMap) {
        let prod_largest_three_basins = mul_largest_three_basins(height_map);
        println!("{}", prod_largest_three_basins);
    }
}
