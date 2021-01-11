#![feature(unchecked_math)]
use std::collections::HashSet;

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mine_locations = get_mine_locations(minefield);

    minefield
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.chars()
                .enumerate()
                .map(|(j, letter)| match letter {
                    '*' => '*',
                    _ => match count_adjacent_mines((i, j), &mine_locations) {
                        0 => ' ',
                        n => std::char::from_digit(n as u32, 10).expect("only valid digits"),
                    },
                })
                .collect()
        })
        .collect()
}

fn count_adjacent_mines(pos: (usize, usize), mine_locations: &HashSet<(usize, usize)>) -> usize {
    (pos.0.saturating_sub(1)..=pos.0 + 1)
        .flat_map(|row| (pos.1.saturating_sub(1)..=pos.1 + 1).map(move |col| (row, col)))
        .filter(|pos| mine_locations.contains(pos))
        .count()
}

fn get_mine_locations(minefield: &[&str]) -> HashSet<(usize, usize)> {
    minefield
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.chars().enumerate().filter_map(
                move |(j, c)| {
                    if c == '*' {
                        Some((i, j))
                    } else {
                        None
                    }
                },
            )
        })
        .collect()
}
