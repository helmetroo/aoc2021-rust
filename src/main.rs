mod puzzles;
mod utils;

use crate::puzzles::p17::P17;
use crate::puzzles::puzzle::Puzzle;

fn main() {
    Puzzle::solve(&P17 {}, false)
}
