pub fn collatz(mut n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }

    let mut steps = 0;
    while n > 1 {
        if n % 2 != 0 {
            n = (n * 3) + 1;
        } else {
            n /= 2;
        }
        steps += 1;
    }

    Some(steps)
}
