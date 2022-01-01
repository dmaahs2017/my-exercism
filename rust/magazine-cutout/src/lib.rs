use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut mag_map: HashMap<&str, usize> =
        magazine.iter().fold(Default::default(), |mut map, word| {
            *map.entry(word).or_default() += 1;
            map
        });

    for word in note {
        if let Some(count) = mag_map.get_mut(word) {
            if *count == 0 {
                return false;
            }
            *count -= 1;
        } else {
            return false;
        }
    }
    true
}
