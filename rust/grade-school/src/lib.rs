use std::collections::{BTreeMap, BTreeSet};

type Grade = u32;
/// Using a BTreeSet keeps the grade roster sorted.
type Roster = BTreeSet<String>;

pub struct School {
    /// Using a BTreeMap keeps the grades sorted
    cohort: BTreeMap<Grade, Roster>,
}

impl Default for School {
    fn default() -> Self {
        Self::new()
    }
}

impl School {
    pub fn new() -> School {
        Self {
            cohort: Default::default(),
        }
    }

    pub fn add(&mut self, grade: Grade, student: &str) {
        self.cohort
            .entry(grade)
            .or_insert_with(Default::default)
            .insert(student.to_string());
    }

    pub fn grades(&self) -> Vec<Grade> {
        self.cohort.keys().cloned().collect()
    }

    pub fn grade(&self, grade: Grade) -> Vec<String> {
        self.cohort
            .get(&grade)
            .map_or(Default::default(), |map| map.iter().cloned().collect())
    }
}
