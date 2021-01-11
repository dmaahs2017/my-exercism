pub fn match_last(stack: &mut Vec<char>, c: char) -> bool {
    match stack.pop() {
        Some(s) => s == c,
        None => false,
    }
}
pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = Vec::new();

    for c in string.chars() {
        match c {
            '(' | '{' | '[' => stack.push(c),
            ')' => {
                if !match_last(&mut stack, '(') {
                    return false;
                }
            }
            '}' => {
                if !match_last(&mut stack, '{') {
                    return false;
                }
            }
            ']' => {
                if !match_last(&mut stack, '[') {
                    return false;
                }
            }
            _ => (),
        }
    }
    stack.is_empty()
}
