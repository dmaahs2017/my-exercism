pub fn reply(message: &str) -> &str {
    let is_question = message.contains("?");
    let is_yelling = message.chars().filter(|c| c.is_ascii_alphabetic()).all(|c| c.is_uppercase());
    let empty = message.chars().filter(|c| !c.is_whitespace()).count() == 0;

    if empty {
        return "Fine. Be that way!"
    } else if is_question && is_yelling {
        return "Calm down, I know what I'm doing!"
    } else if is_question {
        return "Sure."
    } else if is_yelling {
        return "Whoa, chill out!"
    }
    "Whatever."
}
