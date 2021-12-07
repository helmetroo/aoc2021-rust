use std::collections::HashSet;

use itertools::Itertools;

use crate::puzzles::puzzle::Puzzle;

type Indices = Vec<(usize, usize)>;
type Board = [[u32; 5]; 5];
type NumSet = HashSet<u32>;
type IdxSet = HashSet<usize>;

pub struct Subsystem {
    draw_order: Vec<u32>,
    boards: Vec<Board>,
}

struct Winner {
    index: usize,
    marked_nums: NumSet,
    last_number: u32,
}

fn read_boards(raw_data: &[String]) -> Vec<Board> {
    let mut boards = Vec::new();
    let mut cur_board = [[0u32; 5]; 5];
    let mut row: usize = 0;
    let lines = raw_data.len();
    for (index, line) in raw_data.iter().enumerate() {
        let nums: Vec<u32> = line
            .trim()
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        for (col, num) in nums.iter().enumerate() {
            cur_board[row][col] = *num;
        }
        row += 1;

        if nums.len() == 0 || index == lines - 1 {
            boards.push(cur_board.clone());

            // Clear array
            for row in cur_board.iter_mut() {
                for num in row.iter_mut() {
                    *num = 0u32;
                }
            }

            row = 0;
        }
    }

    boards
}

fn find_first_winner(subsystem: &Subsystem) -> Option<Winner> {
    let mut marked_nums = NumSet::new();
    marked_nums.extend(&subsystem.draw_order[0..4]);
    for num in &subsystem.draw_order[4..] {
        marked_nums.insert(*num);
        for (index, board) in subsystem.boards.iter().enumerate() {
            let winner = is_winner(board, &marked_nums);
            if winner {
                return Some(Winner {
                    index,
                    marked_nums,
                    last_number: *num,
                });
            }
        }
    }

    None
}

fn find_last_winner(subsystem: &Subsystem) -> Option<Winner> {
    let mut winners = Vec::new();

    let mut marked_nums = NumSet::new();
    marked_nums.extend(&subsystem.draw_order[0..4]);

    let mut called_boards = IdxSet::new();

    for num in &subsystem.draw_order[4..] {
        if called_boards.len() == subsystem.boards.len() {
            break
        }

        marked_nums.insert(*num);
        for (index, board) in subsystem.boards.iter().enumerate() {
            if called_boards.contains(&index) {
                continue
            }

            let winner = is_winner(board, &marked_nums);
            if winner {
                called_boards.insert(index);
                winners.push(Winner {
                    index,
                    marked_nums: marked_nums.clone(),
                    last_number: *num,
                });
            }
        }
    }

    winners.pop()
}

fn is_winner(board: &Board, marked_nums: &NumSet) -> bool {
    // Two different indices = two different ways to walk through the board (col by col, row by row)
    let indices = (0..5).cartesian_product(0..5);
    let reverse_indices = indices.clone().map(|(row, col)| (col, row)).collect();

    let winner_found_walking_columns = check_winner(board, &indices.collect(), marked_nums);
    let winner_found_walking_rows = check_winner(board, &reverse_indices, marked_nums);
    winner_found_walking_columns || winner_found_walking_rows
}

fn check_winner(board: &Board, indices: &Indices, marked_nums: &NumSet) -> bool {
    let mut remaining_nums: NumSet = HashSet::new();
    remaining_nums.extend(marked_nums.into_iter());

    for (iteration, (row, col)) in indices.iter().enumerate() {
        remaining_nums.remove(&board[*row][*col]);

        if (iteration + 1) % 5 == 0 {
            let found_bingo = marked_nums.len() - remaining_nums.len() >= 5;
            if found_bingo {
                return true;
            }

            remaining_nums.clear();
            remaining_nums.extend(marked_nums.into_iter());
        }
    }

    false
}

fn sum_unmarked_nums(board: &Board, marked_nums: &NumSet) -> u32 {
    let mut unmarked_nums = NumSet::new();
    unmarked_nums.extend(board.iter().flat_map(|row| row.iter()));
    unmarked_nums.difference(marked_nums).sum()
}

fn compute_winner_score(possible_winner: &Option<Winner>, subsystem: &Subsystem) -> Option<u32> {
    possible_winner.as_ref().map(|winner| {
        let winning_board = subsystem.boards[winner.index];
        let total_unmarked = sum_unmarked_nums(&winning_board, &winner.marked_nums);
        total_unmarked * winner.last_number
    })
}

pub struct P4;
impl Puzzle<Subsystem> for P4 {
    fn number(&self) -> u8 {
        4
    }

    fn parse_data(&self, raw_data: &Vec<String>) -> Subsystem {
        let draw_order = raw_data[0].split(',').map(|n| n.parse().unwrap()).collect();
        let boards = read_boards(&raw_data[2..]);
        Subsystem { draw_order, boards }
    }

    fn solve_part_one(&self, subsystem: &Subsystem) {
        let possible_winner = find_first_winner(subsystem);
        let winner_score = compute_winner_score(&possible_winner, subsystem);
        println!(
            "{}",
            winner_score.map_or(String::from("No solution"), |s| s.to_string())
        );
    }

    fn solve_part_two(&self, subsystem: &Subsystem) {
        let possible_winner = find_last_winner(subsystem);
        let winner_score = compute_winner_score(&possible_winner, subsystem);
        println!(
            "{}",
            winner_score.map_or(String::from("No solution"), |s| s.to_string())
        );
    }
}
