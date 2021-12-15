mod puzzles;
mod utils;

use crate::puzzles::p9::P9;
use crate::puzzles::puzzle::Puzzle;

fn main() {
    Puzzle::solve(&P9 {}, false)
}
