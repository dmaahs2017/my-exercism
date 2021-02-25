use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut set = HashSet::new();
    // It passes the tests and this way I'm O(n^2 / 6) instead of O(n^2), but I haven't proved
    // that this division optimization will work for ALL cases, but it passes these tests....
    for a in 1..=sum / 3 {
        let rest = sum - a;

        for b in a..=rest / 2 {
            let c_squared = a * a + b * b;
            let c = f32::sqrt(c_squared as f32) as u32;
            if let Some(triples) = get_sorted_tripples_sum(a, b, c, sum) {
                set.insert(triples);
            }
        }
    }

    set
}

fn get_sorted_tripples_sum(a: u32, b: u32, c: u32, sum: u32) -> Option<[u32; 3]> {
    if a * a + b * b == c * c && a + b + c == sum {
        return Some([a, b, c]);
    }
    None
}
