mod puzzles;
mod utils;

use crate::puzzles::p13::P13;
use crate::puzzles::puzzle::Puzzle;

fn main() {
    Puzzle::solve(&P13 {}, false)
}
