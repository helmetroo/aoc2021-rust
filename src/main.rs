mod puzzles;
mod utils;

use crate::puzzles::p7::P7;
use crate::puzzles::puzzle::Puzzle;

fn main() {
    Puzzle::solve(&P7 {}, false)
}
