pub fn factors(mut n: u64) -> Vec<u64> {
    let mut factors = vec![];
    let mut m = 2;
    while n > 1 {        
        dbg!(n, m, &factors);
        while n % m == 0 {
            factors.push(m);
            n /= m;
        }
        m += 1;
    }
    factors 
}

pub fn factors_iter(mut n: u64) -> Vec<u64> {
    let mut result = Vec::new();

    let mut iter = std::iter::successors(Some(2), |&count| {
        // Edge case: all prime factors have been found
        if n == 1 {
            return None;
        }

        // Edge case: The final value of n is itself a prime factor
        if count as f64 > (n as f64).sqrt() {
            result.push(n);
            return None;
        }

        // Edge case: count is the next prime factor.
        if n % count == 0 {
            result.push(count);
            n /= count;
            return Some(2);
        } 

        Some(count + 1)
    });
    while let Some(_) = iter.next() {}
    return result;
}
