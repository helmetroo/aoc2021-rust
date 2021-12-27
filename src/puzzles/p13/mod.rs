use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;

use crate::puzzles::puzzle::Puzzle;

#[derive(Copy, Clone)]
pub enum Axis {
    X,
    Y,
}

pub enum Instruction {
    Dot { x: usize, y: usize },
    Fold { axis: Axis, position: usize },
}

struct Bounds {
    max_row: usize,
    max_col: usize,
}

impl Bounds {
    pub fn new() -> Self {
        Self {
            max_row: usize::MIN,
            max_col: usize::MIN,
        }
    }
}

type TupleRange = (usize, usize);
type Grid = Vec<Vec<bool>>;
type InstructionParseErr = &'static str;
impl FromStr for Instruction {
    type Err = InstructionParseErr;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref DOT_REGEX: Regex = Regex::new(r"(?P<x>\d+),(?P<y>\d+)").unwrap();
            static ref FOLD_REGEX: Regex =
                Regex::new(r"fold along (?P<axis>x|y)=(?P<position>\d+)").unwrap();
        }

        let dot = DOT_REGEX
            .captures(line)
            .as_ref()
            .map(extract_dot_instruction)
            .flatten();

        let fold = FOLD_REGEX
            .captures(line)
            .as_ref()
            .map(extract_fold_instruction)
            .flatten();

        dot.or(fold).ok_or("Invalid instruction")
    }
}

fn parse_match_as_usize(m: regex::Match) -> Option<usize> {
    m.as_str().parse().ok()
}

fn parse_match_as_axis(m: regex::Match) -> Option<Axis> {
    match m.as_str() {
        "x" => Some(Axis::X),
        "y" => Some(Axis::Y),
        _ => None,
    }
}

fn extract_dot_instruction(captures: &regex::Captures) -> Option<Instruction> {
    let x = captures.name("x").map(parse_match_as_usize).flatten();
    let y = captures.name("y").map(parse_match_as_usize).flatten();

    x.zip(y).map(|(x, y)| Instruction::Dot { x, y })
}

fn extract_fold_instruction(captures: &regex::Captures) -> Option<Instruction> {
    let axis = captures.name("axis").map(parse_match_as_axis).flatten();
    let position = captures
        .name("position")
        .map(parse_match_as_usize)
        .flatten();

    axis.zip(position)
        .map(|(axis, position)| Instruction::Fold { axis, position })
}

fn construct_grid(instructions: &[Instruction]) -> Grid {
    let Bounds { max_row, max_col } = get_grid_bounds(instructions);

    let mut grid = vec![vec![false; max_col + 1]; max_row + 1];
    for instruction in instructions {
        if let Instruction::Dot { x, y } = instruction {
            grid[*y][*x] = true;
        }
    }

    grid
}

fn get_grid_bounds(instructions: &[Instruction]) -> Bounds {
    instructions
        .iter()
        .fold(Bounds::new(), |cur_bounds, instruction| match instruction {
            Instruction::Dot { x, y } => {
                let new_cols = if *x > cur_bounds.max_col {
                    *x
                } else {
                    cur_bounds.max_col
                };

                let new_rows = if *y > cur_bounds.max_row {
                    *y
                } else {
                    cur_bounds.max_row
                };

                Bounds {
                    max_row: new_rows,
                    max_col: new_cols,
                }
            }
            _ => cur_bounds,
        })
}

fn fold_grid(grid: &mut Grid, axis: Axis, position: usize) -> Grid {
    let rows = grid.len();
    let cols = grid[0].len();

    // 2*pos = (pos+1) + (pos-1)
    let along_y_axis = matches!(axis, Axis::Y);
    let row_range = if along_y_axis {
        (
            position + 1,
            if position < rows / 2 {
                2 * position
            } else {
                rows
            },
        )
    } else {
        (0, rows)
    };

    let col_range = if along_y_axis {
        (0, cols)
    } else {
        (
            position + 1,
            if position < cols / 2 {
                2 * position
            } else {
                cols
            },
        )
    };

    let map_row = if along_y_axis {
        |src_row, rows| rows - 1 - src_row
    } else {
        |src_row, _| src_row
    };

    let map_col = if along_y_axis {
        |src_col, _| src_col
    } else {
        |src_col, cols| cols - 1 - src_col
    };

    let mut dest_grid = vec![vec![false; col_range.1]; row_range.1];
    fold_grid_with(
        grid,
        &mut dest_grid,
        rows,
        cols,
        row_range,
        col_range,
        &map_row,
        &map_col,
    );
    dest_grid
}

fn fold_grid_with<F, G>(
    src_grid: &Grid,
    dest_grid: &mut Grid,
    rows: usize,
    cols: usize,
    (min_row, max_row): TupleRange,
    (min_col, max_col): TupleRange,
    map_row: &F,
    map_col: &G,
) where
    F: Fn(usize, usize) -> usize,
    G: Fn(usize, usize) -> usize,
{
    for src_row in min_row..max_row {
        for src_col in min_col..max_col {
            let dest_row = map_row(src_row, rows);
            let dest_col = map_col(src_col, cols);
            dest_grid[dest_row][dest_col] =
                src_grid[dest_row][dest_col] | src_grid[src_row][src_col];
        }
    }
}

fn count_dots(grid: &Grid) -> usize {
    grid.iter().flat_map(|row| row.iter()).fold(
        0,
        |count, cell| {
            if *cell {
                count + 1
            } else {
                count
            }
        },
    )
}

fn dot_count_from_instructions(instructions: &Vec<Instruction>, folds: usize) -> usize {
    let fold_index = instructions
        .iter()
        .position(|instruction| matches!(instruction, Instruction::Fold { .. }))
        .unwrap();
    let dots = &instructions[0..fold_index];
    let folds = &instructions[fold_index..fold_index + folds];

    let mut grid = construct_grid(dots);
    for fold in folds {
        if let Instruction::Fold { axis, position } = fold {
            grid = fold_grid(&mut grid, *axis, *position);
        }
    }

    count_dots(&grid)
}

fn print_grid(grid: &Grid) {
    let rows = grid.len();
    let cols = grid[0].len();

    for row in 0..rows {
        for col in 0..cols {
            if grid[row][col] {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

pub struct P13;
impl Puzzle<Vec<Instruction>> for P13 {
    fn number(&self) -> u8 {
        13
    }

    fn parse_data(&self, raw_data: &Vec<String>) -> Vec<Instruction> {
        raw_data
            .iter()
            .filter_map(|line| {
                if line.is_empty() {
                    None
                } else {
                    Instruction::from_str(line.as_str()).ok()
                }
            })
            .collect()
    }

    fn solve_part_one(&self, instructions: &Vec<Instruction>) {
        let dot_count = dot_count_from_instructions(instructions, 1);
        println!("{}", dot_count);
    }

    fn solve_part_two(&self, _instructions: &Vec<Instruction>) {}
}
