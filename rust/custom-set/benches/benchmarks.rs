#![feature(test)]
extern crate test;

use custom_set::{FastSet, NaiveSet};
use test::Bencher;

#[bench]
fn naive_union(b: &mut Bencher) {
    type Set<T> = NaiveSet<T>;
    let s = Set::new(&[1, 2, 3, 4, 5]);
    let s2 = Set::new(&[3, 4, 5, 6]);
    b.iter(|| s.union(&s2))
}

#[bench]
fn fast13_union(b: &mut Bencher) {
    type Set<T> = FastSet<T, 13>;
    let s = Set::new(&[1, 2, 3, 4, 5]);
    let s2 = Set::new(&[3, 4, 5, 6]);
    b.iter(|| s.union(&s2))
}

#[bench]
fn fast31_union(b: &mut Bencher) {
    type Set<T> = FastSet<T, 13>;
    let s = Set::new(&[1, 2, 3, 4, 5]);
    let s2 = Set::new(&[3, 4, 5, 6]);
    b.iter(|| s.union(&s2))
}

#[bench]
fn naive_add(b: &mut Bencher) {
    type Set<T> = NaiveSet<T>;
    let mut s = Set::new(&[]);
    b.iter(|| {
        for i in 1..10000 {
            s.add(i);
        }
    })
}

#[bench]
fn fast13_add(b: &mut Bencher) {
    type Set<T> = FastSet<T, 13>;
    let mut s = Set::new(&[]);
    b.iter(|| {
        for i in 1..10000 {
            s.add(i);
        }
    })
}

#[bench]
fn fast31_add(b: &mut Bencher) {
    type Set<T> = FastSet<T, 31>;
    let mut s = Set::new(&[]);
    b.iter(|| {
        for i in 1..10000 {
            s.add(i);
        }
    })
}

#[bench]
fn fast13_contains(b: &mut Bencher) {
    type Set<T> = FastSet<T, 13>;
    let mut s = Set::new(&[]);
    for i in 1..1000 {
        s.add(i);
    }

    b.iter(|| {
        for i in 500..1500 {
            s.contains(&i);
        }
    })
}

#[bench]
fn fast31_contains(b: &mut Bencher) {
    type Set<T> = FastSet<T, 31>;
    let mut s = Set::new(&[]);
    for i in 1..1000 {
        s.add(i);
    }

    b.iter(|| {
        for i in 500..1500 {
            s.contains(&i);
        }
    })
}

#[bench]
fn naive_contains(b: &mut Bencher) {
    type Set<T> = NaiveSet<T>;
    let mut s = Set::new(&[]);
    for i in 1..1000 {
        s.add(i);
    }

    b.iter(|| {
        for i in 500..1500 {
            s.contains(&i);
        }
    })
}
