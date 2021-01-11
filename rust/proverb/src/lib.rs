pub fn build_proverb(list: &[&str]) -> String {
    if list.len() == 0 {
        return String::new();
    }

    let mut leading_iter = list.into_iter();
    leading_iter.next();
    let first = list
        .into_iter()
        .zip(leading_iter)
        .fold(String::new(), |acc, (e1, e2)| {
            return acc + &format!("For want of a {} the {} was lost.\n", e1, e2);
        });

    first + &format!("And all for the want of a {}.", list[0])
}
