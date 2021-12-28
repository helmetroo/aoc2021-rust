mod puzzles;
mod utils;

use crate::puzzles::p14::P14;
use crate::puzzles::puzzle::Puzzle;

fn main() {
    Puzzle::solve(&P14 {}, false)
}
