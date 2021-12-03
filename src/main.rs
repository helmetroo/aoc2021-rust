mod utils;
mod puzzles;

use crate::puzzles::puzzle::Puzzle;
use crate::puzzles::p1::P1;

fn main() {
    Puzzle::solve(&P1 {})
}
