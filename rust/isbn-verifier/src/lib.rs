/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    // if there is an 'X' then it must be the last character
    if let Some(index) = isbn.find('X') {
        if index != isbn.len() - 1 {
            return false;
        }
    }

    // Iterate over bytes because its faster. No unicode nonsense.
    let products = isbn
        .as_bytes()
        .iter()
        .rev()
        .filter_map(|&c| {
            if c.is_ascii_digit() {
                return Some((c - b'0') as u32);
            } else if c == b'X' {
                return Some(10);
            }
            None
        })
        .enumerate()
        .map(|(i, value)| (i + 1) as u32 * value)
        .collect::<Vec<_>>();

    products.len() == 10 && products.iter().sum::<u32>() % 11 == 0
}
