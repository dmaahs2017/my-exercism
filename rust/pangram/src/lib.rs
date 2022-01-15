use std::collections::HashSet;
/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    sentence
        .as_bytes()
        .iter()
        .filter_map(|byte| {
            if byte.is_ascii_alphabetic() {
                Some(byte.to_ascii_lowercase())
            } else {
                None
            }
        })
        .collect::<HashSet<u8>>()
        .len()
        == 26
}
