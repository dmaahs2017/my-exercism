const MINE_BYTE: u8 = b'*';

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    (0..minefield.len())
        .map(|row| {
            (0..minefield[row].len())
                .map(|col| match minefield[row].as_bytes()[col] {
                    MINE_BYTE => MINE_BYTE as char,
                    _ => count_to_print_char(count_mines(minefield, row, col)),
                })
                .collect()
        })
        .collect()
}

fn count_to_print_char(count: u8) -> char {
    match count {
        0 => ' ',
        c => (c + b'0') as char,
    }
}

fn count_mines(minefield: &[&str], row: usize, col: usize) -> u8 {
    (row.saturating_sub(1)..=row + 1)
        .filter_map(|row| minefield.get(row))
        .flat_map(|row_string| {
            (col.saturating_sub(1)..=col + 1).filter_map(move |col| row_string.as_bytes().get(col))
        })
        .filter(|c| **c == MINE_BYTE)
        .count() as u8
}
