pub fn nth(n: u32) -> u32 {
    let mut max: usize = 16;
    loop {
        let primes = sieve_of_eratosthenes(max);

        if n as usize > primes.len() {
            max = max * 2;
        } else {
            return primes[n as usize];
        }
    }
}

pub fn nth_slow(n: u32) -> u32 {
    let mut primes = vec![2];

    for num in (3..u32::MAX).step_by(2) {
        if (n as usize) < primes.len() {
            return primes[n as usize];
        }
        
        if is_prime(num, &primes) {
            primes.push(num);
        }
    }
    0
}

fn is_prime(num: u32, primes: &[u32]) -> bool {
    // a prime number is one which is not divisible by all of the primes before it
    primes.iter().all(|p| num % p != 0)
}

// Function to generate N prime numbers using
// Sieve of Eratosthenes
fn sieve_of_eratosthenes(max: usize) -> Vec<u32> {
    // Create a boolean array "IsPrime[0..MAX_SIZE]" and
    // initialize all entries it as true. A value in
    // IsPrime[i] will finally be false if i is
    // Not a IsPrime, else true.
    let mut is_prime: Vec<bool> = vec![true; max];

    let mut p = 2;
    while p * p < max {
        // If IsPrime[p] is not changed, then it is a prime
        if is_prime[p] {
            // Update all multiples of p greater than or
            // equal to the square of it
            // numbers which are multiple of p and are
            // less than p^2 are already been marked.
            let mut i = p * p;
            while i < max {
                is_prime[i] = false;
                i += p
            }
        }
        p += 1;
    }

    // Store all prime numbers
    let mut primes: Vec<u32> = Vec::new();
    for p in 2..max {
        if is_prime[p] {
            primes.push(p as u32);
        }
    }
    primes
}
