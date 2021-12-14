mod puzzles;
mod utils;

use crate::puzzles::p8::P8;
use crate::puzzles::puzzle::Puzzle;

fn main() {
    Puzzle::solve(&P8 {}, false)
}
