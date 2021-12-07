mod puzzles;
mod utils;

use crate::puzzles::p4::P4;
use crate::puzzles::puzzle::Puzzle;

fn main() {
    Puzzle::solve(&P4 {}, false)
}
