pub fn abbreviate(phrase: &str) -> String {
    phrase
        .replace("-", " ")
        .split_whitespace()
        .map(|w| {
            let word = {
                if !w.chars().filter(|c| c.is_alphabetic()).all(|c| c.is_ascii_uppercase()) {
                    let mut c = w.chars();
                    match c.next() {
                        None => String::new(),
                        Some(letter) => letter.to_ascii_uppercase().to_string() + c.as_str(),
                    }
                } else {
                    let w = w.to_ascii_lowercase();
                    let mut c = w.chars();
                    match c.next() {
                        None => String::new(),
                        Some(letter) => letter.to_ascii_uppercase().to_string() + c.as_str(),
                    }
                }
            };
            word.chars()
                .filter(|c| c.is_ascii_uppercase())
                .collect::<String>()
        })
        .collect()
}
