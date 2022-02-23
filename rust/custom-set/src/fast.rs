use std::hash::{ Hash, Hasher };

use std::collections::hash_map::DefaultHasher;

/// The Fast implementation using hashing
#[derive(Debug)]
pub struct FastSet<T, const N: usize> {
    v: Vec<Vec<T>>
}

fn get_hashed_bucket<T: Hash>(e: &T, n: usize) -> usize {
    let mut s = DefaultHasher::new();
    e.hash(&mut s);
    s.finish() as usize % n
}

impl<T, const N: usize> FastSet<T, N> 
where T: Clone + PartialEq + Hash
{

    pub fn new(input: &[T]) -> Self {
        let mut new = Self { v: vec![Vec::with_capacity(4); N] };

        input.iter().cloned().for_each(|e| {
            new.add(e);
        });

        new
    }

    pub fn contains(&self, element: &T) -> bool {
        let b = get_hashed_bucket(element, N);
        self.v[b].contains(element)
    }

    pub fn add(&mut self, e: T) {
        let bucket = get_hashed_bucket(&e, N);
        if !self.v[bucket].contains(&e) {
            self.v[bucket].push(e);
        }
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.iter()
            .all(|e| other.iter().any(|e2| e == e2))
    }

    pub fn is_empty(&self) -> bool {
        self.v.iter().all(|b| b.is_empty())
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.iter()
            .all(|e| !other.iter().any(|e2| e == e2))
    }

    #[must_use]
    pub fn intersection(&self, other: &Self) -> Self {
        let mut new = Self::new(&[]);
        self.iter().chain(other.iter()).cloned()
            .filter(|e| self.contains(e) && other.contains(e))
            .for_each(|e| new.add(e));
        new
    }

    #[must_use]
    pub fn difference(&self, other: &Self) -> Self {
        let mut new = Self::new(&[]);
        self.iter().filter(|e| !other.contains(e)).cloned()
            .for_each(|e| new.add(e));
        new
    }

    #[must_use]
    pub fn union(&self, other: &Self) -> Self {
        let mut new = Self::new(&[]);
        self.iter().chain(other.iter()).cloned()
            .for_each(|e| new.add(e));
        new
    }

    pub fn iter(&self) -> impl Iterator<Item=&T> {
        self.v.iter().flatten()
    }
}

impl<T: Clone + PartialEq + Hash, const N: usize> PartialEq for FastSet<T, N> {
    fn eq(&self, rhs: &Self) -> bool {
        self.is_subset(rhs) && rhs.is_subset(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_hashes_are_always_the_same() {
        let x = 8i32;
        let y = 8i32;
        let mut hasher = DefaultHasher::new();
        x.hash(&mut hasher);
        let x_h = hasher.finish();
        
        let mut hasher = DefaultHasher::new();
        y.hash(&mut hasher);
        let y_h = hasher.finish();

        assert_eq!(x_h, y_h);
    }
}
