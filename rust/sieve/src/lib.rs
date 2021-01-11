pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    // Create a boolean array "IsPrime[0..MAX_SIZE]" and
    // initialize all entries it as true. A value in
    // IsPrime[i] will finally be false if i is
    // Not a IsPrime, else true.
    let mut is_prime: Vec<bool> = vec![true; ( upper_bound + 1 ) as usize];
    let mut p = 2;
    while p * p <= upper_bound {
        // If IsPrime[p] is not changed, then it is a prime
        if is_prime[p as usize] {
            // Update all multiples of p greater than or
            // equal to the square of it
            // numbers which are multiple of p and are
            // less than p^2 are already been marked.
            let mut i = p * p;
            while i <= upper_bound {
                is_prime[i as usize] = false;
                i += p
            }
        }
        p += 1;
    }

    // Store all prime numbers
    let mut primes: Vec<u64> = Vec::new();
    for p in 2..=upper_bound {
        if is_prime[p as usize] {
            primes.push(p as u64);
        }
    }
    primes
}


