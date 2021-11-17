use tokeniser::Tokeniser;

fn main() {
    let tokeniser = Tokeniser::new(String::from("1 + 1 / 607-12"));

    for token in tokeniser {
        print!("{} ", token.get_value());
    }
}
