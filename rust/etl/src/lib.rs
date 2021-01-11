use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut new_format = BTreeMap::new();
    for (&point_value, letters) in h {
        letters.iter().fold(&mut new_format, |acc, l| {
            acc.insert(l.to_ascii_lowercase(), point_value);
            acc
        });
    }
    new_format
}
