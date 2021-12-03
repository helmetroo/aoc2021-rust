mod utils;
mod puzzles;

use crate::puzzles::puzzle::Puzzle;
use crate::puzzles::p2::P2;

fn main() {
    Puzzle::solve(&P2 {}, false)
}
