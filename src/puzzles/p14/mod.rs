use std::collections::HashMap;
use std::str::FromStr;

use itertools::Itertools;

use crate::puzzles::puzzle::Puzzle;
use crate::utils::hashmap::{
    add_to_count, get_extreme_value, increment_count, subtract_from_count,
    Extreme,
};

struct PairInsertionRule {
    pair: String,
    element: char,
}

enum ModificationMethod {
    Increment,
    Decrement,
}

struct Modification {
    pair: String,
    method: ModificationMethod,
    count: u64,
}

type ParsePairInsertionRuleErr = &'static str;
impl FromStr for PairInsertionRule {
    type Err = ParsePairInsertionRuleErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split_str = s.split(" -> ");
        if let [pair, element] = split_str.take(2).collect::<Vec<_>>()[..] {
            Ok(PairInsertionRule {
                pair: String::from(pair),
                element: element.chars().next().unwrap(),
            })
        } else {
            Err("Unable to parse rule")
        }
    }
}

type PairInsertionRules = HashMap<String, char>;
pub struct Manual {
    template: String,
    insertion_rules: PairInsertionRules,
}

type ElementCounts = HashMap<char, u64>;
type PairCounts = HashMap<String, u64>;
type Modifications = Vec<Modification>;

fn insert_polymers_and_count_elems(
    s: &String,
    insertion_rules: &PairInsertionRules,
    steps: usize,
) -> ElementCounts {
    // Build up initial pair counts
    let mut pair_counts =
        s.chars()
            .tuple_windows()
            .fold(PairCounts::new(), |mut counts, (left_elem, right_elem)| {
                let pair = [left_elem, right_elem].iter().collect::<String>();
                increment_count(pair, &mut counts);
                counts
            });

    for _ in 0..steps {
        let modifications = get_modifications(&pair_counts, insertion_rules);
        apply_modifications(&mut pair_counts, &modifications);
    }

    // Sum up counts of each character
    let double_pair_counts =
        pair_counts
            .iter()
            .fold(ElementCounts::new(), |mut elem_counts, (pair, count)| {
                if let [left_elem, right_elem] = pair.chars().take(2).collect::<Vec<_>>()[..] {
                    add_to_count(left_elem, *count, &mut elem_counts);
                    add_to_count(right_elem, *count, &mut elem_counts);
                }

                elem_counts
            });

    double_pair_counts
        .iter()
        .fold(ElementCounts::new(), |mut final_counts, (pair, &count)| {
            let final_count = if count % 2 == 1 {
                (count / 2) + 1
            } else {
                count / 2
            };
            final_counts.insert(*pair, final_count);
            final_counts
        })
}

fn get_modifications(
    pair_counts: &PairCounts,
    insertion_rules: &PairInsertionRules,
) -> Modifications {
    let mut modifications = Modifications::new();
    for (pair, count) in pair_counts {
        let new_elem = insertion_rules.get(pair);
        if new_elem.is_some() {
            modifications.push(Modification {
                pair: pair.to_string(),
                method: ModificationMethod::Decrement,
                count: *count,
            });

            let middle_elem = new_elem.unwrap();

            let left_pair = [*middle_elem, pair.chars().nth(1).unwrap()]
                .iter()
                .collect::<String>();
            modifications.push(Modification {
                pair: left_pair,
                method: ModificationMethod::Increment,
                count: *count,
            });

            let right_pair = [pair.chars().nth(0).unwrap(), *middle_elem]
                .iter()
                .collect::<String>();
            modifications.push(Modification {
                pair: right_pair,
                method: ModificationMethod::Increment,
                count: *count,
            });
        }
    }

    modifications
}

fn apply_modifications(pair_counts: &mut PairCounts, modifications: &Modifications) {
    modifications
        .iter()
        .fold(pair_counts, |counts, modification| {
            let count = modification.count;

            if matches!(modification.method, ModificationMethod::Decrement) {
                let pair = modification.pair.clone();
                subtract_from_count(pair, count, counts);
            } else {
                let pair = modification.pair.clone();
                add_to_count(pair, count, counts);
            }

            counts
        });
}

fn diff_btw_extreme_elems(counts: &ElementCounts) -> u64 {
    let max_elem = get_extreme_value(counts, Extreme::Max);
    let min_elem = get_extreme_value(counts, Extreme::Min);

    max_elem.zip(min_elem).map(|(max, min)| max - min).unwrap()
}

fn insert_polymers_and_get_diff_btw_extremes(manual: &Manual, steps: usize) -> u64 {
    let counts = insert_polymers_and_count_elems(&manual.template, &manual.insertion_rules, steps);
    diff_btw_extreme_elems(&counts)
}

pub struct P14;
impl Puzzle<Manual> for P14 {
    fn number(&self) -> u8 {
        14
    }

    fn parse_data(&self, raw_data: &Vec<String>) -> Manual {
        let template = raw_data[0].to_string();
        let insertion_rule_strs = &raw_data[2..];
        let insertion_rules =
            insertion_rule_strs
                .iter()
                .fold(PairInsertionRules::new(), |mut rules, rule| {
                    if let Ok(PairInsertionRule { pair, element }) =
                        PairInsertionRule::from_str(&rule.to_owned())
                    {
                        rules.insert(pair, element);
                        rules
                    } else {
                        rules
                    }
                });

        Manual {
            template,
            insertion_rules,
        }
    }

    fn solve_part_one(&self, manual: &Manual) {
        let diff = insert_polymers_and_get_diff_btw_extremes(manual, 10);
        println!("{}", diff);
    }

    fn solve_part_two(&self, manual: &Manual) {
        let diff = insert_polymers_and_get_diff_btw_extremes(manual, 40);
        println!("{}", diff);
    }
}
