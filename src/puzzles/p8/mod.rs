use std::collections::HashSet;
use std::str::FromStr;

use crate::puzzles::puzzle::Puzzle;

type CharSet = HashSet<char>;
type Patterns = [CharSet; 10];
type Output = [CharSet; 4];

pub struct Note {
    patterns: Patterns,
    output: Output,
}

fn is_unique_digit(digit: &CharSet) -> bool {
    let num_lit = digit.len();

    // Digits 1, 4, 7, 8
    num_lit == 2 || num_lit == 4 || num_lit == 3 || num_lit == 7
}

impl Note {
    pub fn count_unique_digits(&self) -> usize {
        self.output
            .iter()
            .filter(|&digit| is_unique_digit(digit))
            .count()
    }
}

impl FromStr for Note {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokenized_str: Vec<&str> = s.split('|').collect();
        let patterns_split = tokenized_str[0].trim().split(' ');
        let mut patterns: Patterns = Patterns::default();
        for (index, pattern) in patterns_split.into_iter().enumerate() {
            patterns[index] = CharSet::from_iter(pattern.chars());
        }

        let output_split = tokenized_str[1].trim().split(' ');
        let mut output: Output = Output::default();
        for (index, pattern) in output_split.into_iter().enumerate() {
            output[index] = CharSet::from_iter(pattern.chars());
        }

        Ok(Note { patterns, output })
    }
}

pub struct P8;
impl Puzzle<Vec<Note>> for P8 {
    fn number(&self) -> u8 {
        8
    }

    fn parse_data(&self, raw_data: &Vec<String>) -> Vec<Note> {
        raw_data
            .into_iter()
            .map(|line| Note::from_str(&line[0..]).unwrap())
            .collect()
    }

    fn solve_part_one(&self, notes: &Vec<Note>) {
        let count: usize = notes
            .into_iter()
            .map(|note| note.count_unique_digits())
            .sum();
        println!("{}", count);
    }

    fn solve_part_two(&self, _notes: &Vec<Note>) {}
}
