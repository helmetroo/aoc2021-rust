mod puzzles;
mod utils;

use crate::puzzles::p15::P15;
use crate::puzzles::puzzle::Puzzle;

fn main() {
    Puzzle::solve(&P15 {}, false)
}
