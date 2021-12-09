mod puzzles;
mod utils;

use crate::puzzles::p5::P5;
use crate::puzzles::puzzle::Puzzle;

fn main() {
    Puzzle::solve(&P5 {}, false)
}
