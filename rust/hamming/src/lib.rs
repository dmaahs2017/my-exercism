pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() {
        return None;
    }
    // Use bytes instead of Chars because detecting unicode char boundaries is more expensive.
    Some(
        s1.as_bytes()
            .iter()
            .zip(s2.as_bytes())
            .filter(|(a, b)| a != b)
            .count(),
    )
}
