use std::cmp::Ordering;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::ops::Sub;
use std::str::FromStr;

use crate::puzzles::puzzle::Puzzle;

type Decoder<'a> = HashMap<&'a Pattern, u8>;
type Patterns = [Pattern; 10];
type Output = [Pattern; 4];

#[derive(Eq, Clone, Copy)]
struct Pattern {
    hash: u8,
    length: u8,
}

fn char_to_bit(chr: char) -> u8 {
    match chr {
        'a' => 1 << 0,
        'b' => 1 << 1,
        'c' => 1 << 2,
        'd' => 1 << 3,
        'e' => 1 << 4,
        'f' => 1 << 5,
        'g' => 1 << 6,
        _ => 0,
    }
}

fn count_bits(hash: u8) -> u8 {
    (0..8)
        .fold((0, hash), |(bits, cur_hash), _| {
            (if cur_hash & 1 == 1 { bits + 1 } else { bits }, cur_hash >> 1)
        })
        .0
}

impl Pattern {
    pub fn empty() -> Self {
        Pattern { hash: 0, length: 0 }
    }

    pub fn new(pattern_str: &str) -> Self {
        let hash = Pattern::hash_pattern(pattern_str);
        let length = count_bits(hash);
        Pattern { hash, length }
    }

    fn hash_pattern(pattern: &str) -> u8 {
        pattern.chars().fold(0, |hash, chr| hash | char_to_bit(chr))
    }

    pub fn is_unique_digit(&self) -> bool {
        let num_lit = self.length;

        // Digits 1, 4, 7, 8
        num_lit == 2 || num_lit == 4 || num_lit == 3 || num_lit == 7
    }
}

impl Sub<Pattern> for Pattern {
    type Output = Pattern;

    fn sub(self, rhs: Pattern) -> Pattern {
        let new_hash = self.hash & !rhs.hash;
        let new_length = count_bits(new_hash);
        Pattern {
            hash: new_hash,
            length: new_length,
        }
    }
}

impl Hash for Pattern {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.hash.hash(state);
    }
}

impl Ord for Pattern {
    fn cmp(&self, other: &Self) -> Ordering {
        self.length.cmp(&other.length)
    }
}

impl PartialOrd for Pattern {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.length.cmp(&other.length))
    }
}

impl PartialEq for Pattern {
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash
    }
}

pub struct NoteEntry {
    patterns: Patterns,
    output: Output,
}

impl NoteEntry {
    pub fn count_unique_digits(&self) -> u64 {
        self.output
            .iter()
            .filter(|&pattern| pattern.is_unique_digit())
            .count()
            .try_into()
            .unwrap()
    }

    pub fn get_output_value(&self) -> u64 {
        let mut decoder = Decoder::new();

        let pattern_for_1 = &self.patterns[0];
        let pattern_for_7 = &self.patterns[1];
        let pattern_for_4 = &self.patterns[2];
        let pattern_for_8 = &self.patterns[9];
        // Hint from https://www.reddit.com/r/adventofcode/comments/rbvpui/2021_day_8_part_2_my_logic_on_paper_i_used_python/?utm_source=share&utm_medium=web2x&context=3
        // to use the L structure found diffing 1 from 4
        let pattern_for_4_and_1 = *pattern_for_4 - *pattern_for_1;

        decoder.insert(pattern_for_1, 1);
        decoder.insert(pattern_for_7, 7);
        decoder.insert(pattern_for_4, 4);
        decoder.insert(pattern_for_8, 8);

        // Deduce the 5 segmented numbers (2, 3, 5)
        for index in 3..=5 {
            let current = &self.patterns[index];
            let includes_1 = (*current - *pattern_for_1).length == 3;
            if includes_1 {
                decoder.insert(current, 3);
            } else {
                let includes_4_and_1_diff = (*current - pattern_for_4_and_1).length == 3;
                decoder.insert(current, if includes_4_and_1_diff { 5 } else { 2 });
            }
        }

        // Deduce the 6 segmented numbers (0, 6, 9)
        for index in 6..=8 {
            let current = &self.patterns[index];
            let includes_4 = (*current - *pattern_for_4).length == 2;
            if includes_4 {
                decoder.insert(current, 9);
            } else {
                let includes_4_and_1_diff = (*current - pattern_for_4_and_1).length == 4;
                decoder.insert(current, if includes_4_and_1_diff { 6 } else { 0 });
            }
        }

        self.output
            .iter()
            .fold((0u64, 4u64), |(value, exp), pattern| {
                let power = u64::pow(10u64, (exp - 1).try_into().unwrap());
                let base = decoder[pattern] as u64;
                (value + base * power, exp - 1)
            })
            .0
    }
}

impl FromStr for NoteEntry {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokenized_str: Vec<&str> = s.split('|').collect();
        let patterns_split = tokenized_str[0].trim().split(' ');
        let mut patterns: Patterns = [Pattern::empty(); 10];
        for (index, pattern) in patterns_split.into_iter().enumerate() {
            patterns[index] = Pattern::new(pattern);
        }
        patterns.sort();

        let output_split = tokenized_str[1].trim().split(' ');
        let mut output: Output = [Pattern::empty(); 4];
        for (index, pattern) in output_split.into_iter().enumerate() {
            output[index] = Pattern::new(pattern);
        }

        Ok(NoteEntry { patterns, output })
    }
}

pub struct P8;
impl Puzzle<Vec<NoteEntry>> for P8 {
    fn number(&self) -> u8 {
        8
    }

    fn parse_data(&self, raw_data: &Vec<String>) -> Vec<NoteEntry> {
        raw_data
            .into_iter()
            .map(|line| NoteEntry::from_str(&line[0..]).unwrap())
            .collect()
    }

    fn solve_part_one(&self, notes: &Vec<NoteEntry>) {
        let count: u64 = notes
            .into_iter()
            .map(|entry| entry.count_unique_digits())
            .sum();
        println!("{}", count);
    }

    fn solve_part_two(&self, notes: &Vec<NoteEntry>) {
        let count: u64 = notes
            .into_iter()
            .map(|entry| entry.get_output_value())
            .sum();
        println!("{}", count);
    }
}
