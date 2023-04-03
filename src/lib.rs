use std::cmp::Ordering;

mod tests;

pub fn find<T: AsRef<[U]>, U: Clone + Ord>(array: T, key: U) -> Option<usize> {
    let mut items: Vec<U> = array.as_ref().into();
    if items.is_empty() || items.len() == 1 && items[0] != key {
        return None;
    }
    items.sort_unstable();
    let index = items.len() / 2;
    match key.cmp(&items[index]) {
        Ordering::Equal => return Some(index),
        Ordering::Less => return find(&items[..index], key),
        Ordering::Greater => return find(&items[index..], key).map(|i| index + i),
    }
}
