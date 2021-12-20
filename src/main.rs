mod puzzles;
mod utils;

use crate::puzzles::p11::P11;
use crate::puzzles::puzzle::Puzzle;

fn main() {
    Puzzle::solve(&P11 {}, false)
}
