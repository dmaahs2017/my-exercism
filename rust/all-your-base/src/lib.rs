#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base <= 1 {
        return Err(Error::InvalidInputBase);
    }
    if to_base <= 1 {
        return Err(Error::InvalidOutputBase);
    }

    let mut n = to_base_10(number, from_base)?;
    let mut ans = vec![];

    if n == 0 {
        ans.push(0);
    }
    while n > 0 {
        ans.push(n % to_base);
        n /= to_base;
    }

    ans.reverse();
    Ok(ans)
}

fn to_base_10(number: &[u32], base: u32) -> Result<u32, Error> {
    if let Some(d) = number.iter().filter(|&&d| d >= base).next() {
        return Err(Error::InvalidDigit(*d));
    }

    Ok(number
        .iter()
        .rev()
        .enumerate()
        .map(|(i, d)| base.pow(i as u32) * d)
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn as_base_10_works() {
        let n = &[1, 0, 0, 0]; // 8
        assert_eq!(to_base_10(n, 2).unwrap(), 8);
    }
}
