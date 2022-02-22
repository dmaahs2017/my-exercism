#[derive(Debug)]
pub enum Error {
    Unknown,
}

type Notes = [&'static str; 12];
const SHARPS: Notes = [
    "A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#",
];

const FLATS: Notes = [
    "A", "Bb", "B", "C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab",
];

/// Usize = half step
enum Interval {
    Chromatic,
    Pattern(Vec<usize>),
}

pub struct Scale {
    notes: Notes,
    root: usize, // index of NOTES
    interval: Interval,
}

impl Scale {
    // tonic = start note
    pub fn new(tonic: &str, intervals: &str) -> Result<Scale, Error> {
        let notes = if use_flats(tonic) { FLATS } else { SHARPS };

        let root = get_root_idx(tonic, notes);

        let interval = Interval::Pattern(
            intervals
                .chars()
                .map(|c| match c {
                    'M' => 2,
                    'm' => 1,
                    'A' => 3,
                    _ => unreachable!(),
                })
                .collect(),
        );

        Ok(Scale {
            notes,
            root,
            interval,
        })
    }

    pub fn chromatic(tonic: &str) -> Result<Scale, Error> {
        let notes = if use_flats(tonic) { FLATS } else { SHARPS };

        let root = get_root_idx(tonic, notes);

        Ok(Scale {
            notes,
            root,
            interval: Interval::Chromatic,
        })
    }

    pub fn enumerate(&self) -> Vec<String> {
        match &self.interval {
            Interval::Chromatic => self
                .notes
                .iter()
                .cycle()
                .skip(self.root)
                .take(13)
                .map(|s| s.to_string())
                .collect(),
            Interval::Pattern(pattern_interval) => {
                let mut iter = self.notes.iter().cycle().skip(self.root);
                pattern_interval
                    .iter()
                    .map(|interval| {
                        let note = iter.next().unwrap().to_string();
                        for _ in 1..*interval {
                            iter.next();
                        }
                        note
                    })
                    .chain(std::iter::once(self.notes[self.root].to_string()))
                    .collect()
            }
        }
    }
}

fn use_flats(tonic: &str) -> bool {
    let flat_tonics = [
        "F", "Bb", "Eb", "Ab", "Db", "Gb", "d", "g", "c", "f", "bb", "eb",
    ];

    return dbg!(flat_tonics.contains(&tonic));
}

fn get_root_idx(tonic: &str, notes: Notes) -> usize {
    notes
        .iter()
        .enumerate()
        .find_map(|(i, t)| {
            if *t.to_ascii_uppercase() == tonic.to_ascii_uppercase() {
                Some(i)
            } else {
                None
            }
        })
        .unwrap()
}
