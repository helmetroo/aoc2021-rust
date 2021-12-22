mod puzzles;
mod utils;

use crate::puzzles::p12::P12;
use crate::puzzles::puzzle::Puzzle;

fn main() {
    Puzzle::solve(&P12 {}, false)
}
