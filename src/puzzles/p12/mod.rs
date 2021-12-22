use std::collections::HashMap;
use std::fmt::{Debug, Formatter, Result};
use std::hash::{Hash, Hasher};

use crate::puzzles::puzzle::Puzzle;

#[derive(PartialEq, Eq, Clone)]
enum NodeCategory {
    SmallCave,
    BigCave,
    Start,
    End,
}

#[derive(Eq, Clone)]
pub struct Node {
    id: String,
    category: NodeCategory,
}

impl Node {
    pub fn new(id: &str) -> Self {
        let category = Node::get_category(id);
        Self {
            id: id.to_owned(),
            category,
        }
    }

    pub fn start() -> Self {
        Node::new("start")
    }

    pub fn end() -> Self {
        Node::new("end")
    }

    pub fn is_small_cave(&self) -> bool {
        matches!(self.category, NodeCategory::SmallCave) || self.is_start() || self.is_end()
    }

    pub fn is_start(&self) -> bool {
        matches!(self.category, NodeCategory::Start)
    }

    pub fn is_end(&self) -> bool {
        matches!(self.category, NodeCategory::End)
    }

    fn get_category(id: &str) -> NodeCategory {
        match id {
            "start" => NodeCategory::Start,
            "end" => NodeCategory::End,
            _ => Node::determine_cave_size(id),
        }
    }

    fn determine_cave_size(id: &str) -> NodeCategory {
        if id.chars().next().unwrap().is_uppercase() {
            NodeCategory::BigCave
        } else {
            NodeCategory::SmallCave
        }
    }
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id) && self.category.eq(&other.category)
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str(&self.id)
    }
}

type AdjacencyGraph = HashMap<Node, Vec<Node>>;
type Visited = HashMap<Node, u32>;

fn count_paths_recur(graph: &AdjacencyGraph) -> u32 {
    count_paths_from(&Node::start(), graph, &Visited::new())
}

fn count_paths_from(current: &Node, graph: &AdjacencyGraph, visited: &Visited) -> u32 {
    if visited.contains_key(&Node::end()) {
        1
    } else {
        let mut visited_with_next = visited.clone();
        increment_count(current.clone(), &mut visited_with_next);

        let no_adjacents = Vec::new();

        let next_adjacents = graph
            .get(current)
            .unwrap_or(&no_adjacents)
            .into_iter()
            .filter_map(|adj| {
                let mut visited_with_adj = visited_with_next.clone();
                let new_count = increment_count(adj.clone(), &mut visited_with_adj);
                let invalid_path = new_count >= 2 && adj.is_small_cave();
                if invalid_path {
                    None
                } else {
                    Some((adj.clone(), visited_with_adj))
                }
            });

        next_adjacents.into_iter().fold(0, |count, (adj, visited)| {
            count + count_paths_from(&adj, &graph, &visited)
        })
    }
}

fn increment_count(node: Node, visited: &mut Visited) -> u32 {
    let new_count = visited
        .entry(node)
        .and_modify(|count| *count += 1)
        .or_insert(1);

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
                let nodes = split_line.into_iter().map(Node::new).collect::<Vec<Node>>();

                // Can't impl Copy because Node contains a String
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
        let path_count = count_paths_recur(map);
        println!("{}", path_count);
    }

    fn solve_part_two(&self, _map: &AdjacencyGraph) {}
}
