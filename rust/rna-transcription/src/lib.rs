#[derive(Debug, PartialEq)]
pub struct Dna {
    strand: String,
}

#[derive(Debug, PartialEq)]
pub struct Rna {
    strand: String,
}

fn is_dna_nucleotide(n: char) -> bool {
    match n {
        'A' | 'C' | 'G' | 'T' => true,
        _ => false,
    }
}

fn is_rna_nucleotide(n: char) -> bool {
    match n {
        'A' | 'C' | 'G' | 'U' => true,
        _ => false,
    }
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        if let Some((i, _)) = dna
            .chars()
            .enumerate()
            .find(|&(_, n)| !is_dna_nucleotide(n))
        {
            return Err(i);
        }

        Ok(Dna {
            strand: dna.to_string(),
        })
    }

    pub fn into_rna(self) -> Rna {
        Rna {
            strand: self
                .strand
                .chars()
                .map(|n| match n {
                    'G' => 'C',
                    'C' => 'G',
                    'T' => 'A',
                    'A' => 'U',
                    _ => unreachable!(),
                })
                .collect(),
        }
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        if let Some((i, _)) = rna
            .chars()
            .enumerate()
            .find(|&(_, n)| !is_rna_nucleotide(n))
        {
            return Err(i);
        }
        Ok(Rna {
            strand: rna.to_string(),
        })
    }
}
