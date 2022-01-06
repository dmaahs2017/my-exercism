use std::collections::HashSet;
pub fn check(candidate: &str) -> bool {
    let mut set = HashSet::new();
    candidate
        .as_bytes()
        .iter()
        .all(|c| match c.to_ascii_lowercase() {
            b' ' | b'-' => true,
            lower_letter => set.insert(lower_letter),
        })
}
