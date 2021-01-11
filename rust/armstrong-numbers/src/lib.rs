pub fn is_armstrong_number(num: u32) -> bool {
    let mut digits = Vec::new();
    let mut t = num;
    while t > 0 {
        digits.push(t % 10);
        t = t / 10;
    }

    digits.iter().map(|&n| n.pow(digits.len() as u32)).sum::<u32>() == num
}
