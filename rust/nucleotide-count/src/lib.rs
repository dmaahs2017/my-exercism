use std::collections::HashMap;

fn is_nucleotide(c: char) -> bool {
    match c {
        'A' | 'C' | 'T' | 'G' => true,
        _ => false,
    }
}

fn validate(dna: &str) -> Result<Vec<char>, char> {
    let (nucs, nots): (Vec<char>, Vec<char>) = dna.chars().partition(|&c| is_nucleotide(c));

    if let Some(&invalid_char) = nots.first() {
        return Err(invalid_char);
    }

    Ok(nucs)
}

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !is_nucleotide(nucleotide) {
        return Err(nucleotide);
    }

    let nucs = validate(dna)?;

    Ok(nucs.iter().filter(|&&c| c == nucleotide).count())
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let nucs = validate(dna)?;

    let mut map = HashMap::new();
    map.insert('A', 0);
    map.insert('C', 0);
    map.insert('T', 0);
    map.insert('G', 0);

    Ok(nucs.iter().fold(map, |mut acc, &c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    }))
}
