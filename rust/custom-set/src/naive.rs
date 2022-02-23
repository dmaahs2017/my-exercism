#![allow(unused)]

/// The Easy Naive Implementation
#[derive(Debug)]
pub struct NaiveSet<T> {
    v: Vec<T>
}

impl<T: Clone + PartialEq> NaiveSet<T> {
    /// Naive Imple
    pub fn new(input: &[T]) -> Self {
        Self {v: input.to_vec()}
    }

    /// Naive Impl: O(n)
    pub fn contains(&self, element: &T) -> bool {
        self.v.contains(element)
    }

    /// Naive Impl: O(n)
    pub fn add(&mut self, e: T) {
        if !self.contains(&e) {
            self.v.push(e);
        }
    }

    /// Naive Impl: O(n^2)
    pub fn is_subset(&self, other: &Self) -> bool {
        self.v.iter().all(|e| other.contains(e))
    }

    /// Naive Impl: O(1)
    pub fn is_empty(&self) -> bool {
        self.v.is_empty()
    }

    /// Naive Impl: O(n^2)
    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.v.iter().all(|e| !other.contains(e))
    }

    /// Naive Impl: O(n^2)
    #[must_use]
    pub fn intersection(&self, other: &Self) -> Self {
        let mut new = Self::new(&[]);
        self.v.iter().chain(other.v.iter())
            .filter(|e| self.contains(e) && other.contains(e))
            .cloned()
            .for_each(|e| new.add(e));

        new
    }

    /// Naive Impl: O(n^2)
    #[must_use]
    pub fn difference(&self, other: &Self) -> Self {
        let mut new = Self::new(&[]);
        self.v.iter()
            .filter(|e| !other.contains(e))
            .cloned()
            .for_each(|e| new.add(e));

        new
    }

    #[must_use]
    /// Naive Impl: O(n^2)
    pub fn union(&self, other: &Self) -> Self {
        let mut new = Self::new(&[]);
        self.v.iter().chain(other.v.iter())
            .cloned()
            .for_each(|e| new.add(e));

        new
    }
}

impl<T: Clone + PartialEq> PartialEq for NaiveSet<T> {
    fn eq(&self, rhs: &Self) -> bool {
        self.is_subset(rhs) && rhs.is_subset(self)
    }
}
