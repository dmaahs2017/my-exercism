use std::collections::HashMap;
const CODON_LEN: usize = 3;
const STOP_CODON: &str = "stop codon";

pub struct CodonsInfo<'a> {
    pairs: HashMap<&'a str, &'a str>,
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        self.pairs.get(codon).map(|c| *c)
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        let mut protiens = Vec::new();


        for i in (0..rna.len()).step_by(CODON_LEN) {
            if i + CODON_LEN > rna.len() {
                return None;
            }

            let codon_str = &rna[i..i+CODON_LEN];
            if let Some(&protien) = self.pairs.get(codon_str) {
                if protien == STOP_CODON {
                    break;
                }
                protiens.push(protien)
            } else {
                return None;
            }
        }

        Some(protiens)
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    CodonsInfo {
        pairs: pairs.into_iter().collect(),
    }
}
