use core::fmt;
use std::fs;
use std::io;
pub struct Lexer {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Lexer {
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        &self.tokens
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '{' => self.add_token(TokenType::LeftCurlyBracket, None),
            '}' => self.add_token(TokenType::RightCurlyBracket, None),
            '[' => self.add_token(TokenType::LeftCurlyBracket, None),
            ']' => self.add_token(TokenType::RightCurlyBracket, None),
            ':' => self.add_token(TokenType::Colon, None),
            ',' => self.add_token(TokenType::Comma, None),
            '\n' => {
                self.line = self.line + 1;
            }
            '"' => {
                self.string();
            }
            c if c.is_ascii_digit() => {
                self.number();
            }
            ' ' | '\r' | '\t' => {}

            _ => report("Unexpected character"),
        }
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<Value>) {
        let text: String = self.source[self.start..self.current].iter().collect();
        self.tokens.push(Token {
            token_type,
            lexeme: String::from(text),
            literal: literal.unwrap_or(Value::Null),
        })
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

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line = self.line + 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            report("Unterminated string.");
            return;
        }
        self.advance();

        let literal = self.source[self.start + 1..self.current - 1]
            .iter()
            .collect();
        self.add_token(TokenType::String, Some(Value::String(literal)));
    }

    fn number(&mut self) {
        while self.peek().is_ascii_digit() && !self.is_at_end() {
            self.advance();
        }
        let string_digit: String = self.source[self.start..self.current].iter().collect();
        let number = string_digit.parse::<i32>().expect("Unable to parse digit");
        self.add_token(TokenType::Number, Some(Value::Number(number)))
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
    Number,
}

pub enum Value {
    String(String),
    Number(i32),
    Array(Vec<Value>),
    Bool(bool),
    Null,
}

pub struct Parser;

pub fn run_file(path: &str) {
    let contents = fs::read_to_string(path).expect("Unable to read file");
    let mut lexer = Lexer::new(contents);
    let tokens = lexer.scan_tokens();
    for token in tokens {
        println!("{:?}", token.lexeme);
    }
}

pub fn run_prompt() {
    loop {
        let mut prompt = String::new();
        io::stdin()
            .read_line(&mut prompt)
            .expect("Failed to read line");

        let mut lexer = Lexer::new(prompt.to_owned());
        let tokens = lexer.scan_tokens();
        for token in tokens {
            println!("{:?}", token.lexeme);
        }
    }
}

pub fn report(e: &str) {
    println!("{e}");
}

#[cfg(test)]
mod step_1 {
    use super::*;

    #[test]
    fn valid_json_file() {
        let contents = fs::read_to_string("tests/step1/valid.json").expect("Unable to read file");
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
        let contents = fs::read_to_string("tests/step1/valid2.json").expect("Unable to read file");
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
