pub struct Luhn {
    s: String,
}

impl Luhn {
    pub fn is_valid(&self) -> bool {
        self.s
            .chars()
            .rev()
            .filter(|c| !c.is_whitespace())
            .try_fold((0, 0), |(sum, c), d| {
                d.to_digit(10)
                    .map(|n| if c % 2 == 1 { n * 2 } else { n })
                    .map(|n| if n > 9 { n - 9 } else { n })
                    .map(|n| (n + sum, c + 1))
            })
            .map_or(false, |(sum, count)| sum % 10 == 0 && count > 1)
    }
}

impl<T> From<T> for Luhn
where
    T: std::fmt::Display,
{
    fn from(input: T) -> Self {
        Self {
            s: input.to_string(),
        }
    }
}
