#[warn(clippy::unwrap_used, clippy::expect_used)]
#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidNote(String),
    Undeterminiate,
}

type ScaleResult<T> = Result<T, Error>;

type Notes = [&'static str; 12];
const SHARPS: Notes = [
    "A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#",
];

const FLATS: Notes = [
    "A", "Bb", "B", "C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab",
];

/// Note is u8 because the note should never exceed 11
type Note = u8;
pub struct Scale {
    flats: bool,
    notes: Vec<Note>,
}

fn note_string(note: Note, flats: bool) -> String {
    if flats {
        FLATS[note as usize].to_string()
    } else {
        SHARPS[note as usize].to_string()
    }
}

fn string_to_note(note_str: &str) -> ScaleResult<Note> {
    let invalid_note_fn = || Error::InvalidNote(note_str.to_string());
    let mut byte_iter = note_str.as_bytes().iter();

    let mut sum = match byte_iter
        .next()
        .ok_or_else(invalid_note_fn)?
        .to_ascii_uppercase()
    {
        b @ (b'A' | b'B') => (b - b'A') as i8 * 2,
        b @ (b'C'..=b'E') => (b - b'A') as i8 * 2 - 1,
        b @ (b'F' | b'G') => (b - b'A') as i8 * 2 - 2,
        _ => return Err(invalid_note_fn()),
    };

    sum += if let Some(b) = byte_iter.next() {
        match b {
            b'#' => 1,
            b'b' => -1,
            _ => return Err(invalid_note_fn()),
        }
    } else {
        0
    };

    if byte_iter.next().is_some() {
        return Err(invalid_note_fn());
    }

    // A flat should be half step 11
    if sum < 0 {
        sum += 12;
    }

    Ok(sum as Note)
}

impl Scale {
    // tonic = start note
    pub fn new(tonic: &str, intervals: &str) -> ScaleResult<Scale> {
        let flats = use_flats(tonic);
        let root = string_to_note(tonic)?;

        let notes = intervals
            .as_bytes()
            .iter()
            .fold((root, vec![root]), |(mut note, mut acc), b| {
                match b {
                    b'm' => note += 1,
                    b'M' => note += 2,
                    b'A' => note += 3,
                    _ => unreachable!(),
                }
                note %= 12;
                acc.push(note);
                (note, acc)
            })
            .1;

        Ok(Self { flats, notes })
    }

    pub fn chromatic(tonic: &str) -> ScaleResult<Self> {
        let flats = use_flats(tonic);
        let root = string_to_note(tonic)?;
        let notes = (0..12).cycle().skip(root as usize).take(13).collect();

        Ok(Scale { flats, notes })
    }

    pub fn enumerate(&self) -> Vec<String> {
        self.notes
            .iter()
            .map(|note| note_string(*note, self.flats))
            .collect()
    }
}

fn use_flats(tonic: &str) -> bool {
    let flat_tonics = [
        "F", "Bb", "Eb", "Ab", "Db", "Gb", "d", "g", "c", "f", "bb", "eb",
    ];

    flat_tonics.contains(&tonic)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_to_note_works() {
        let str_note = "A";
        let note = string_to_note(str_note).unwrap();
        assert_eq!(note, 0);

        let str_note = "A#";
        let note = string_to_note(str_note).unwrap();
        assert_eq!(note, 1, "sharps work");

        let str_note = "Bb";
        let note = string_to_note(str_note).unwrap();
        assert_eq!(note, 1, "flats work");

        let str_note = "B";
        let note = string_to_note(str_note).unwrap();
        assert_eq!(note, 2);

        let str_note = "C";
        let note = string_to_note(str_note).unwrap();
        assert_eq!(note, 3);

        let str_note = "C#";
        let note = string_to_note(str_note).unwrap();
        assert_eq!(note, 4);

        let str_note = "Eb";
        let note = string_to_note(str_note).unwrap();
        assert_eq!(note, 6);

        let str_note = "E";
        let note = string_to_note(str_note).unwrap();
        assert_eq!(note, 7);

        let str_note = "F";
        let note = string_to_note(str_note).unwrap();
        assert_eq!(note, 8);

        let str_note = "F#";
        let note = string_to_note(str_note).unwrap();
        assert_eq!(note, 9);

        let str_note = "G";
        let note = string_to_note(str_note).unwrap();
        assert_eq!(note, 10);

        let str_note = "G#";
        let note = string_to_note(str_note).unwrap();
        assert_eq!(note, 11);

        let str_note = "Ab";
        let note = string_to_note(str_note).unwrap();
        assert_eq!(note, 11);
    }

    #[test]
    fn new_test() {
        let s = Scale::new("B", "mMA").unwrap();
        assert_eq!(s.notes, vec![2, 3, 5, 8]);
    }
}
