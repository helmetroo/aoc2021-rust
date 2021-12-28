use std::collections::HashMap;
use std::str::FromStr;

use itertools::Itertools;

use crate::puzzles::puzzle::Puzzle;
use crate::utils::hashmap::{get_extreme_value, increment_count, Extreme};

struct PairInsertionRule {
    pair: String,
    element: char,
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

type PolymerCounts = HashMap<char, u32>;
type PolymerInsertion = (String, PolymerCounts);

fn insert_polymers_and_count_elems(
    s: &String,
    insertion_rules: &PairInsertionRules,
) -> PolymerInsertion {
    let mut handled_first_pair = false;

    s.chars().tuple_windows().fold(
        (String::new(), PolymerCounts::new()),
        |(mut cur_str, mut counts), (left_elem, right_elem)| {
            let pair = [left_elem, right_elem].iter().collect::<String>();
            let new_elem = insertion_rules.get(&pair);

            if !handled_first_pair {
                increment_count(left_elem, &mut counts);
                cur_str.push(left_elem);

                handled_first_pair = true;
            }

            if new_elem.is_some() {
                let middle_elem = *new_elem.unwrap();
                increment_count(middle_elem, &mut counts);
                cur_str.push(middle_elem);
            }

            increment_count(right_elem, &mut counts);
            cur_str.push(right_elem);

            (cur_str, counts)
        },
    )
}

fn insert_polymers_and_count_elems_from(manual: &Manual, steps: usize) -> PolymerCounts {
    let mut seed_str = manual.template.clone();
    let mut counts = PolymerCounts::new();
    for _ in 0..steps {
        let (next_seed_str, next_counts) =
            insert_polymers_and_count_elems(&seed_str, &manual.insertion_rules);
        seed_str = next_seed_str;
        counts = next_counts;
    }

    counts
}

fn diff_btw_extreme_elems(counts: &PolymerCounts) -> u32 {
    let max_elem = get_extreme_value(counts, Extreme::Max);
    let min_elem = get_extreme_value(counts, Extreme::Min);

    max_elem
        .zip(min_elem)
        .map(|(max, min)| {
            max - min
        })
        .unwrap()
}

fn insert_polymers_and_get_diff_btw_extremes(manual: &Manual, steps: usize) -> u32 {
    let counts = insert_polymers_and_count_elems_from(manual, steps);
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

    fn solve_part_two(&self, _manual: &Manual) {}
}
