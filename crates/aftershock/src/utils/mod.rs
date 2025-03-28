use std::collections::HashMap;

pub mod datetime;

pub fn group_by<T, K, V>(
    items: Vec<T>,
    key_selector: impl Fn(&T) -> K,
    value_selector: impl Fn(&T) -> V,
) -> HashMap<K, Vec<V>>
where
    K: std::hash::Hash + Eq,
    T: Clone,
{
    items.into_iter().fold(HashMap::new(), |mut acc, item| {
        let key = key_selector(&item);
        let value = value_selector(&item);

        acc.entry(key).or_insert_with(Vec::new).push(value);

        acc
    })
}
