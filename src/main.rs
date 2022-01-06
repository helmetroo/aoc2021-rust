mod puzzles;
mod utils;

use crate::puzzles::p16::P16;
use crate::puzzles::puzzle::Puzzle;

fn main() {
    Puzzle::solve(&P16 {}, false)
}
