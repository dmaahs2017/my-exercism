use std::collections::HashMap;
const STOP_CODON: &str = "stop codon";

pub struct CodonsInfo<'a> {
    pairs: HashMap<&'a str, &'a str>,
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        self.pairs.get(codon).cloned()
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        rna.as_bytes()
            .chunks(3)
            .map(std::str::from_utf8)
            .map(|codon| self.name_for(codon.expect("Invalid Utf8 bytes")))
            .take_while(|&protien| protien != Some(STOP_CODON))
            .collect()
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    CodonsInfo {
        pairs: pairs.into_iter().collect(),
    }
}
