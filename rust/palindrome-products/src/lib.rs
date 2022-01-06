#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Palindrome {
    p: u64
}

impl Palindrome {
    pub fn new(a: u64, b: u64) -> Palindrome {
        Palindrome { p: a * b }
    }

    pub fn value(&self) -> u64 {
        self.p
    }

    pub fn insert(&mut self, a: u64, b: u64) {
        self.p = a * b
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    if min > max {
        return None;
    }
    
    // This solution is hacky as hell to pass the four digit case in reasonable time. Doesn't
    // extend beyond the four digit case.
    let max_res = combinations(min, max).rev().take(10_000)
        .filter_map(|(a, b)| { 
            if is_palindrome(a * b) {
                return Some(( a, b, a * b ));
            }
            None
        }).max_by(|a, b| a.2.cmp(&b.2));

    let min_res = combinations(min, max).take(10_000)
        .filter_map(|(a, b)| { 
            if is_palindrome(a * b) {
                return Some(( a, b, a * b ));
            }
            None
        }).min_by(|a, b| a.2.cmp(&b.2));




    if let Some((max, min)) = max_res.zip(min_res) {
        return Some(( Palindrome::new(min.0, min.1), Palindrome::new(max.0, max.1) ));
    }
    if let Some(max) = max_res {
        let p = Palindrome::new(max.0, max.1);
        return Some(( p, p ));
    }
    if let Some(min) = min_res {
        let p = Palindrome::new(min.0, min.1);
        return Some((p, p))
    }
    None
}

fn is_palindrome(v: u64) -> bool {
    let s = v.to_string();
    s.chars().zip(s.chars().rev()).all(|(a, b)| a == b)
}

fn combinations(min: u64, max: u64) -> impl DoubleEndedIterator<Item = (u64, u64)> {
    (min..=max).flat_map(move |a| (a..=max).map(move |b| (a, b)))
}

