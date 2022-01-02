pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len == 0 {
        return vec![Default::default(); digits.len() + 1];
    }

    digits
        .as_bytes()
        .windows(len)
        .map(|bytes| bytes.iter().map(|b| *b as char).collect())
        .collect()
}
