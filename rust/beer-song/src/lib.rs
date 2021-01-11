pub fn verse(n: u32) -> String {
    if n == 0 {
        return "No more bottles of beer on the wall, no more bottles of beer.
Go to the store and buy some more, 99 bottles of beer on the wall.\n".to_string();
    }
    if n == 1 {
        return "1 bottle of beer on the wall, 1 bottle of beer.
Take it down and pass it around, no more bottles of beer on the wall.\n".to_string();
    }
    if n == 2 {
        return "2 bottles of beer on the wall, 2 bottles of beer.
Take one down and pass it around, 1 bottle of beer on the wall.\n".to_string();
    }

    format!("{} bottles of beer on the wall, {} bottles of beer.
Take one down and pass it around, {} bottles of beer on the wall.\n", n, n, n-1)
}

pub fn sing(start: u32, end: u32) -> String {
    let mut s = String::new();
    let mut i = start;
    while i > end {
        s += &verse(i);
        if i != end {
            s += "\n";
        }
        i -= 1;
    }
    s += &verse(end);
    s
}
