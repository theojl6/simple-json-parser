use std::fs;
pub struct Lexer {
    source: Vec<char>,
    tokens: Option<Vec<Token>>,
    start: usize,
    current: usize,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Lexer {
            source: source.chars().collect(),
            tokens: Some(Vec::new()),
            start: 0,
            current: 0,
        }
    }

    pub fn scan_tokens(&mut self) -> Option<Vec<Token>> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.take()
    }

    fn is_at_end(&mut self) -> bool {
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
            ' ' | '\r' | '\t' => {}
            _ => report("Unexpected character"),
        }
    }

    fn add_token(&mut self, token_type: TokenType) {
        let text: String = self.source[self.start..self.current].iter().collect();
        self.tokens.as_mut().expect("add token").push(Token {
            token_type,
            lexeme: String::from(text),
            literal: Value::Null,
        });
    }

    fn advance(&mut self) -> char {
        let c = self.source[self.current];
        self.current = self.current + 1;
        c
    }
}

pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Value,
}

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
