use std::collections::{HashMap, HashSet};

fn hashset_with_digits() -> HashSet<u8> {
    let mut hs = HashSet::new();
    for i in 0..10 {
        hs.insert(i);
    }
    hs
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let mut map = input
        .chars()
        .filter_map(|c| {
            if c.is_alphabetic() {
                Some((c, hashset_with_digits()))
            } else {
                None
            }
        })
        .collect::<HashMap<char, _>>();

    let words = input
        .split_whitespace()
        .filter_map(|s| {
            let s = s.trim();
            if s.chars().all(|c| c.is_alphabetic()) {
                Some(s)
            } else {
                None
            }
        })
        .collect::<Vec<&str>>();

    // characters at the start cannot be 0
    for word in words {
        let first = word.chars().next().unwrap();
        map.entry(first).and_modify(|set| {
            set.remove(&0);
        });
    }

    dbg!(&map);
    while map.values().any(|set| set.len() > 1) {
        todo!()
    }

    map.into_iter()
        .map(|(c, mut set)| {
            let v = set
                .drain()
                .next()
                .expect("There must exist one value in the set at this point");
            Some((c, v))
        })
        .collect()
}

fn to_numbers(x: Vec<String>, values: &HashMap<char, u8>) -> Vec<u32> {
    x.iter()
        .map(|s| {
            s.chars()
                .map(|c| format!("{}", values.get(&c).unwrap()))
                .collect::<String>()
                .parse()
                .unwrap()
        })
        .collect()
}

fn test(left: Vec<String>, right: Vec<String>, values: &HashMap<char, u8>) -> bool {
    let left: u32 = to_numbers(left, values).iter().sum();
    let right: u32 = to_numbers(right, values).iter().sum();

    left == right
}
