use std::io;
use std::io::BufRead;
use forth::Forth;
fn main() {
    let mut f = Forth::new();
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        f.eval(&line).unwrap();
        println!("{}", f);
    }
}
