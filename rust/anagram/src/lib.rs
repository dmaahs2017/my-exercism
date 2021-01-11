use std::collections::{HashMap, HashSet};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let freq_map = freq_map_from_word(&word);

    let mut anagrams = HashSet::new();

    for &possible_anagram in possible_anagrams.iter() {
        if word.to_uppercase() == possible_anagram.to_uppercase() {
            continue;
        }

        let anagram_freq_map = freq_map_from_word(&possible_anagram);
        if anagram_freq_map == freq_map {
            anagrams.insert(possible_anagram);
        }
    }

    anagrams
}

fn freq_map_from_word(word: &str) -> HashMap<String, u32> {
    word.chars().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c.to_uppercase().to_string()).or_insert(0) += 1;
        acc
    })
}
