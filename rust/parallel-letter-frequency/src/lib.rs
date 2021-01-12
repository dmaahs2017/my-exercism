use std::collections::HashMap;
use std::thread;

const MIN_WORKERS: usize = 5;

type CharFreq = HashMap<char, usize>;

pub fn frequency(input: &[&str], worker_count: usize) -> CharFreq {
    let worker_count = input.len() / worker_count + 1;
    if worker_count < MIN_WORKERS {
        return count_letters(input);
    }

    let workers: Vec<_> = input
        .chunks(worker_count)
        .map(|chunk| {
            let lines: Vec<String> = chunk.iter().map(|s| s.to_string()).collect();
            thread::spawn(move || count_letters(&lines))
        })
        .collect();

    let mut map = HashMap::new();
    for w in workers {
        merge(&mut map, &w.join().unwrap());
    }
    map
}

fn merge(dest: &mut CharFreq, src: &CharFreq) {
    for (&k, v) in src {
        *dest.entry(k).or_insert(0) += v;
    }
}

fn count_letters<S: AsRef<str>>(lines: &[S]) -> CharFreq {
    lines
        .iter()
        .flat_map(|s| {
            s.as_ref()
                .chars()
                .map(|c| c.to_ascii_lowercase())
                .filter(|c| !c.is_ascii_punctuation())
                .filter(|c| !c.is_digit(10))
        })
        .fold(HashMap::new(), |mut map, c| {
            *map.entry(c).or_insert(0) += 1;
            map
        })
}
