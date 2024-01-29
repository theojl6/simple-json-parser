use ::simple_json_parser::{read_file, Lexer};
fn main() {
    let contents = read_file("tests/step1/valid.json");
    let mut lexer = Lexer::new(contents);
    let tokens = lexer.scan_tokens().expect("Tokens are gone!");
    for token in tokens {
        println!("{:?}", token.lexeme);
    }
}
