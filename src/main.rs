use std::io;

use tokeniser::parser::Expression;
use tokeniser::token::Token;
use tokeniser::Tokeniser;

fn main() {
    loop {
        let mut s = String::new();
        let result = io::stdin().read_line(&mut s).unwrap();

        if result == 0 {
            break;
        }

        let tokens: Vec<Token> = Tokeniser::new(s).collect();
        let expression: Expression = tokens.into();

        println!("{}", expression);
    }
}
