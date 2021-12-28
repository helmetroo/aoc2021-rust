use std::collections::HashMap;
use std::hash::Hash;

pub enum Extreme {
    Min,
    Max,
}

pub fn increment_count<T: Eq + Hash>(item: T, map: &mut HashMap<T, u32>) -> u32 {
    let new_count = map.entry(item).and_modify(|count| *count += 1).or_insert(1);
    *new_count
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
