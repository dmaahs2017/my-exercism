pub struct Luhn {
    digits: Option<Vec<u8>>,
}

impl Luhn {
    pub fn is_valid(&self) -> bool {
        self.digits.as_ref().map_or(false, |digits| {
            let (sum, count) = digits
                .iter()
                .rev()
                .cloned()
                .fold((0, 0), |(sum, c), mut d| {
                    if c % 2 == 1 {
                        d *= 2
                    };
                    if d > 9 {
                        d -= 9
                    };
                    (d + sum, c + 1)
                });
            sum % 10 == 0 && count > 1
        })
    }
}

/// Here is the example of how the From trait could be implemented
/// for the &str type. Naturally, you can implement this trait
/// by hand for the every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<T> From<T> for Luhn
where
    T: std::fmt::Display,
{
    fn from(input: T) -> Self {
        Self {
            digits: input
                .to_string()
                .chars()
                .filter(|c| !c.is_whitespace())
                .map(|c| c.to_digit(10).map(|d| d as u8))
                .collect(),
        }
    }
}
