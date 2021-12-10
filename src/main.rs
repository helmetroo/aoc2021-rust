mod puzzles;
mod utils;

use crate::puzzles::p6::P6;
use crate::puzzles::puzzle::Puzzle;

fn main() {
    Puzzle::solve(&P6 {}, false)
}
