use std::collections::HashMap;
use std::fmt::{Debug, Formatter, Result};
use std::hash::{Hash, Hasher};

use crate::puzzles::puzzle::Puzzle;

#[derive(PartialEq, Eq, Clone)]
enum CaveCategory {
    SmallCave,
    BigCave,
    Start,
    End,
}

enum Mode {
    One,
    Two,
}

#[derive(Eq, Clone)]
pub struct Cave {
    id: String,
    category: CaveCategory,
}

impl Cave {
    pub fn new(id: &str) -> Self {
        let category = Cave::get_category(id);
        Self {
            id: id.to_owned(),
            category,
        }
    }

    pub fn start() -> Self {
        Cave::new("start")
    }

    pub fn end() -> Self {
        Cave::new("end")
    }

    pub fn is_small(&self) -> bool {
        matches!(self.category, CaveCategory::SmallCave)
    }

    pub fn is_big(&self) -> bool {
        matches!(self.category, CaveCategory::BigCave)
    }

    pub fn is_start(&self) -> bool {
        matches!(self.category, CaveCategory::Start)
    }

    fn get_category(id: &str) -> CaveCategory {
        match id {
            "start" => CaveCategory::Start,
            "end" => CaveCategory::End,
            _ => Cave::determine_cave_size(id),
        }
    }

    fn determine_cave_size(id: &str) -> CaveCategory {
        if id.chars().next().unwrap().is_uppercase() {
            CaveCategory::BigCave
        } else {
            CaveCategory::SmallCave
        }
    }
}

impl Hash for Cave {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for Cave {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id) && self.category.eq(&other.category)
    }
}

impl Debug for Cave {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str(&self.id)
    }
}

type AdjacencyGraph = HashMap<Cave, Vec<Cave>>;
type Visited = HashMap<Cave, u32>;

fn count_paths(graph: &AdjacencyGraph, mode: Mode) -> u32 {
    if matches!(mode, Mode::One) {
        count_paths_strict(&Cave::start(), graph, &Visited::new())
    } else {
        count_paths_loose(&Cave::start(), graph, &Visited::new(), false)
    }
}

fn count_paths_strict(current: &Cave, graph: &AdjacencyGraph, visited: &Visited) -> u32 {
    if visited.contains_key(&Cave::end()) {
        1
    } else {
        let mut visited_with_next = visited.clone();
        increment_count(current.clone(), &mut visited_with_next);

        let no_adjacents = Vec::new();
        let adjacents = graph
            .get(current)
            .unwrap_or(&no_adjacents)
            .into_iter()
            .filter_map(|adj| {
                let mut visited_with_adj = visited_with_next.clone();
                let new_count = increment_count(adj.clone(), &mut visited_with_adj);
                let valid_path = new_count < 2 || adj.is_big();

                if valid_path {
                    Some((adj.clone(), visited_with_adj))
                } else {
                    None
                }
            });

        adjacents.into_iter().fold(0, |count, (adj, visited)| {
            count + count_paths_strict(&adj, &graph, &visited)
        })
    }
}

/*
  Yeah, this one kinda got me. The trick is to keep track of a separate parameter (permission to visit a small cave again).
  A little bit of help from these sources to sort out the logic...
    - https://www.reddit.com/r/adventofcode/comments/rg081o/2021_day_12_part_2_python_stuck_on_part_2_for/
    - https://www.reddit.com/r/adventofcode/comments/rehj2r/comment/hob10my/?utm_source=share&utm_medium=web2x&context=3
*/
fn count_paths_loose(
    current: &Cave,
    graph: &AdjacencyGraph,
    visited: &Visited,
    granted_second_visit: bool,
) -> u32 {
    if visited.contains_key(&Cave::end()) {
        1
    } else {
        let mut visited_with_next = visited.clone();
        increment_count(current.clone(), &mut visited_with_next);

        let no_adjacents = Vec::new();
        graph
            .get(current)
            .unwrap_or(&no_adjacents)
            .into_iter()
            .filter_map(|adj| {
                let mut visited_with_adj = visited_with_next.clone();
                let visit_count = increment_count(adj.clone(), &mut visited_with_adj);

                if adj.is_small() {
                    if visit_count == 1 {
                        Some((adj.clone(), visited_with_adj, granted_second_visit))
                    } else if !granted_second_visit {
                        Some((adj.clone(), visited_with_adj, true))
                    } else {
                        None
                    }
                } else if adj.is_start() {
                    if visit_count <= 1 {
                        Some((adj.clone(), visited_with_adj, granted_second_visit))
                    } else {
                        None
                    }
                } else {
                    Some((adj.clone(), visited_with_adj, granted_second_visit))
                }
            })
            .fold(0, |count, (adj, visited, grant_second_visit)| {
                count + count_paths_loose(&adj, &graph, &visited, grant_second_visit)
            })
    }
}

fn increment_count<T: Eq + Hash>(item: T, map: &mut HashMap<T, u32>) -> u32 {
    let new_count = map.entry(item).and_modify(|count| *count += 1).or_insert(1);
    *new_count
}

pub struct P12;
impl Puzzle<AdjacencyGraph> for P12 {
    fn number(&self) -> u8 {
        12
    }

    fn parse_data(&self, raw_data: &Vec<String>) -> AdjacencyGraph {
        raw_data
            .iter()
            .fold(AdjacencyGraph::new(), |mut graph, line| {
                let split_line: Vec<&str> = line.split('-').collect();
                let nodes = split_line.into_iter().map(Cave::new).collect::<Vec<Cave>>();

                // Can't impl Copy because Cave contains a String
                let from = nodes[0].clone();
                let to = nodes[1].clone();
                let adjacents = graph.entry(to).or_insert(Vec::new());
                adjacents.push(from);

                let from = nodes[0].clone();
                let to = nodes[1].clone();
                let adjacents = graph.entry(from).or_insert(Vec::new());
                adjacents.push(to);

                graph
            })
    }

    fn solve_part_one(&self, map: &AdjacencyGraph) {
        let path_count = count_paths(map, Mode::One);
        println!("{}", path_count);
    }

    fn solve_part_two(&self, map: &AdjacencyGraph) {
        let path_count = count_paths(map, Mode::Two);
        println!("{}", path_count);
    }
}
