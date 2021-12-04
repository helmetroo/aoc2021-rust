mod puzzles;
mod utils;

use crate::puzzles::p3::P3;
use crate::puzzles::puzzle::Puzzle;

fn main() {
    Puzzle::solve(&P3 {}, false)
}
