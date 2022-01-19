pub fn encode(source: &str) -> String {
    source
        .chars()
        // add extra byte because the last byte doesn't get folded into the string
        .chain(std::iter::once('\0'))
        .fold(
            Default::default(),
            |(mut string, last_char, count): (String, Option<char>, usize), c| {
                if let Some(last_char) = last_char {
                    if c == last_char {
                        return (string, Some(c), count + 1);
                    }
                    if count > 1 {
                        string.push_str(&count.to_string());
                    }
                    string.push(last_char);
                    return (string, Some(c), 1);
                }

                (string, Some(c), 1)
            },
        )
        .0
}

pub fn decode(source: &str) -> String {
    source
        .chars()
        .fold(
            Default::default(),
            |(mut decoded, mut num_builder): (String, String), c| {
                if c.is_digit(10) {
                    num_builder.push(c);
                    return (decoded, num_builder);
                }

                if let Ok(num) = num_builder.parse::<usize>() {
                    for _ in 0..num {
                        decoded.push(c)
                    }
                } else {
                    decoded.push(c)
                }
                num_builder.clear();
                (decoded, num_builder)
            },
        )
        .0
}
