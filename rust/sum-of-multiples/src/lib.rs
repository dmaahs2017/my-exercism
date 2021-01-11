pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let is_factor_of = |n: u32, factors: &[u32]| -> bool {
        factors.iter().fold(false, |acc, &factor| {
            if factor == 0 {
                return acc;
            }
            acc || n % factor == 0
        })
    };

    (1..limit).filter(|&i| is_factor_of(i, factors)).sum()
}
