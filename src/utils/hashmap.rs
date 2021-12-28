use std::collections::HashMap;
use std::hash::Hash;

pub enum Extreme {
    Min,
    Max,
}

pub fn add_to_count<T: Eq + Hash>(item: T, to_add: u64, map: &mut HashMap<T, u64>) -> u64 {
    let new_count = map
        .entry(item)
        .and_modify(|count| *count = count.checked_add(to_add).unwrap_or(u64::MAX))
        .or_insert(to_add);
    *new_count
}

pub fn subtract_from_count<T: Eq + Hash>(item: T, to_sub: u64, map: &mut HashMap<T, u64>) -> u64 {
    let new_count = map
        .entry(item)
        .and_modify(|count| *count = count.checked_sub(to_sub).unwrap_or(0))
        .or_insert(0);
    *new_count
}

pub fn increment_count<T: Eq + Hash>(item: T, map: &mut HashMap<T, u64>) -> u64 {
    add_to_count(item, 1, map)
}

pub fn get_extreme_value<K, V: Ord + Copy>(map: &HashMap<K, V>, extreme: Extreme) -> Option<V> {
    if matches!(extreme, Extreme::Max) {
        map.iter()
            .max_by(|(_, a), (_, b)| a.cmp(&b))
            .map(|(_, v)| *v)
    } else {
        map.iter()
            .min_by(|(_, a), (_, b)| a.cmp(&b))
            .map(|(_, v)| *v)
    }
}
