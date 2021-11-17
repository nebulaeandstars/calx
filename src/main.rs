use std::io;

use tokeniser::Tokeniser;

fn main() {
    loop {
        let mut s = String::new();
        let result = io::stdin().read_line(&mut s).unwrap();

        if result == 0 {
            break;
        }

        let tokeniser = Tokeniser::new(s);
        for token in tokeniser {
            print!("{} ", token.get_value());
        }
        println!();
    }
}
