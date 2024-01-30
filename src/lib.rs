use std::fs;
pub struct Lexer {
    source: Vec<char>,
    tokens: Option<Vec<Token>>,
    start: usize,
    current: usize,
    line: usize,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Lexer {
            source: source.chars().collect(),
            tokens: Some(Vec::new()),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.take().unwrap_or_default()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '{' => self.add_token(TokenType::LeftCurlyBracket),
            '}' => self.add_token(TokenType::RightCurlyBracket),
            '[' => self.add_token(TokenType::LeftCurlyBracket),
            ']' => self.add_token(TokenType::RightCurlyBracket),
            ':' => self.add_token(TokenType::Colon),
            ',' => self.add_token(TokenType::Comma),
            '\n' => {
                self.line = self.line + 1;
            }
            ' ' | '\r' | '\t' => {}

            _ => report("Unexpected character"),
        }
    }

    fn add_token(&mut self, token_type: TokenType) {
        let text: String = self.source[self.start..self.current].iter().collect();
        match self.tokens.as_mut() {
            Some(tokens) => tokens.push(Token {
                token_type,
                lexeme: String::from(text),
                literal: Value::Null,
            }),
            None => {}
        }
    }

    fn advance(&mut self) -> char {
        let c = self.source[self.current];
        self.current = self.current + 1;
        c
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source[self.current]
    }
}

pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Value,
}

#[derive(PartialEq, Debug)]
pub enum TokenType {
    LeftCurlyBracket,
    RightCurlyBracket,
    Colon,
    Comma,
    LeftSquareBracket,
    RightSquareBracket,
    String,
}

pub enum Value {
    String(String),
    Number(i32),
    Array(Vec<Value>),
    Null,
}

pub struct Parser;

pub fn read_file(path: &str) -> String {
    let contents = fs::read_to_string(path).expect("Unable to read file");
    contents
}

pub fn report(e: &str) {
    println!("{e}");
}

#[cfg(test)]
mod step_1 {
    use super::*;

    #[test]
    fn valid_json_file() {
        let contents = read_file("tests/step1/valid.json");
        let mut lexer = Lexer::new(contents);
        let tokens = lexer.scan_tokens();
        let mut token_iter = tokens.iter();

        assert_eq!(
            token_iter.next().unwrap().token_type,
            TokenType::LeftCurlyBracket
        );
        assert_eq!(
            token_iter.next().unwrap().token_type,
            TokenType::RightCurlyBracket
        );
    }

    #[test]
    fn valid_2_json_file() {
        let contents = read_file("tests/step1/valid2.json");
        let mut lexer = Lexer::new(contents);
        let tokens = lexer.scan_tokens();
        let mut token_iter = tokens.iter();

        assert_eq!(
            token_iter.next().unwrap().token_type,
            TokenType::LeftCurlyBracket
        );
        assert_eq!(
            token_iter.next().unwrap().token_type,
            TokenType::RightCurlyBracket
        );
    }
}
