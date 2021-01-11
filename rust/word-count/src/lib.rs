use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    words
        .to_lowercase()
        .unicode_words()
        .fold(HashMap::new(), |mut map, word| {
            *map.entry(word.to_string()).or_insert(0) += 1;
            map
        })
}
