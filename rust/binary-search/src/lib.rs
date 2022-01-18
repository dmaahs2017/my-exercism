use std::cmp::Ord;
use std::cmp::Ordering;

/// Binary search. Returns None if element is not found. Otherwise returns Some(index)
pub fn find<C, T>(array: C, key: T) -> Option<usize>
where
    C: AsRef<[T]>,
    T: Ord,
{
    let slice = array.as_ref();
    let mid = get_mid(slice)?;
    match key.cmp(&slice[mid]) {
        Ordering::Equal => return Some(mid),
        Ordering::Greater => return Some(1 + mid + find(&slice[mid+1..], key)?),
        Ordering::Less => return find(&slice[..mid], key),
    }
}

/// Fails if array is empty
fn get_mid<T>(arr: &[T]) -> Option<usize> {
    Some((arr.len().checked_sub(1)?) / 2)
}
