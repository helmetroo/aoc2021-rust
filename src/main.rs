mod puzzles;
mod utils;

use crate::puzzles::p10::P10;
use crate::puzzles::puzzle::Puzzle;

fn main() {
    Puzzle::solve(&P10 {}, false)
}
