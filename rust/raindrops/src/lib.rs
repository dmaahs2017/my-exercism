pub fn raindrops(n: u32) -> String {
    if n % 3 != 0 && n % 5 != 0 && n % 7 != 0 {
        return n.to_string()
    }

    let mut builder = String::with_capacity(15); //init w/ max size to avoid reallocations
    if n % 3 == 0 {
        builder += "Pling";
    }
    if n % 5 == 0 {
        builder += "Plang";
    }
    if n % 7 == 0 {
        builder += "Plong";
    }
    builder
}
