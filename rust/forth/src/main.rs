use std::io::{self, Write };
use forth::Forth;
fn main() {
    let mut f = Forth::new();
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    loop {
        stdout.write(b"> ").unwrap();
        stdout.flush().unwrap();
        stdin.read_line(&mut buffer).unwrap();
        f.eval(&buffer).unwrap();
        buffer.clear();
        println!("{}", &f);
    }
        
}
