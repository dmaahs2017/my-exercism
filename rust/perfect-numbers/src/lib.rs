use std::cmp::Ordering;
#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    }
    Some(match alliquot_sum(num).cmp(&num) {
        Ordering::Equal => Classification::Perfect,
        Ordering::Greater => Classification::Abundant,
        Ordering::Less => Classification::Deficient,
    })
}

fn alliquot_sum(num: u64) -> u64 {
    factors(num).sum()
}

/// Iterates over factors of num, not including num itself.
fn factors(num: u64) -> impl Iterator<Item = u64> {
    // The largest factor can be no greater than num / 2
    (1..=num / 2).filter(move |x| num % x == 0)
}
