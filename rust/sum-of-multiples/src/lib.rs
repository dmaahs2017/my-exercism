pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit)
        .filter(|&i| factors.iter().any(|&f| f != 0 && i % f == 0))
        .sum()
}
