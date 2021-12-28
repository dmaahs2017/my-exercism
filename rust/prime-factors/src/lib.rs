pub fn factors(mut n: u64) -> Vec<u64> {
    let mut result = Vec::new();

    let mut iter = std::iter::successors(Some(2), |&count| {
        if n == 1 {
            return None;
        }

        if count as f64 > (n as f64).sqrt() {
            result.push(n);
            return None;
        }

        if n % count == 0 {
            result.push(count);
            n /= count;
            Some(2)
        } else {
            Some(count + 1)
        }
    });
    while let Some(_) = iter.next() {}
    return result;
}
