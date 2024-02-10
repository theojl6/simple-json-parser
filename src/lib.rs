use std::collections::HashMap;
use std::fs;
use std::io;
pub struct Lexer {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    keywords: HashMap<String, TokenType>,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        let mut keywords = HashMap::new();
        keywords.insert(String::from("null"), TokenType::Null);
        keywords.insert(String::from("true"), TokenType::True);
        keywords.insert(String::from("false"), TokenType::False);
        Lexer {
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            keywords,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token {
            token_type: TokenType::Eof,
            lexeme: String::from(""),
            literal: Value::Null,
        });
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
            '[' => self.add_token(TokenType::LeftSquareBracket, None),
            ']' => self.add_token(TokenType::RightSquareBracket, None),
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
            a if a.is_ascii_alphabetic() => {
                self.identifier();
            }

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

    fn identifier(&mut self) {
        while self.peek().is_ascii_alphanumeric() {
            self.advance();
        }
        let text: String = self.source[self.start..self.current].iter().collect();
        let token_type = self.keywords.get(&text);
        match token_type {
            Some(t) => {
                self.add_token(t.clone(), None);
            }
            None => report("Unexpected character."),
        }
    }
}

pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Value,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum TokenType {
    LeftCurlyBracket,
    RightCurlyBracket,
    Colon,
    Comma,
    LeftSquareBracket,
    RightSquareBracket,
    String,
    Number,
    Null,
    True,
    False,
    Eof,
}

#[derive(Debug, Clone)]
pub enum Value {
    String(String),
    Number(i32),
    Array(Vec<Value>),
    Bool(bool),
    Null,
    Object(Box<[(String, Value)]>),
}

// pair -> string ":" value
// value -> string | number | "null" | "true" | "false" | object | array
// object -> "{" (pair ",")* "}"
// array -> "[" (literal ",")* "]"
pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Value {
        self.expression()
    }

    fn expression(&mut self) -> Value {
        if self.matches(Box::new([TokenType::LeftCurlyBracket])) {
            return self.object();
        }
        return self.value();
    }

    fn value(&mut self) -> Value {
        if self.matches(Box::new([TokenType::False])) {
            return Value::Bool(false);
        }
        if self.matches(Box::new([TokenType::True])) {
            return Value::Bool(true);
        }
        if self.matches(Box::new([TokenType::Number, TokenType::String])) {
            return self.previous().literal.clone();
        }
        if self.matches(Box::new([TokenType::Null])) {
            return Value::Null;
        }
        if self.matches(Box::new([TokenType::LeftSquareBracket])) {
            return self.array();
        }
        report("Unrecognized value");
        return Value::Null;
    }

    fn object(&mut self) -> Value {
        let mut pairs = Vec::new();
        while self.matches(Box::new([TokenType::String, TokenType::Colon])) {
            let key = self.previous().literal.clone();
            let key_string = match key {
                Value::String(s) => s,
                _ => {
                    report("Something went wrong.");
                    String::from("")
                }
            };
            self.advance();
            let value = self.expression();
            pairs.push((key_string, value));
            if self.check(&TokenType::Comma) {
                self.advance();
            }
        }
        if self.previous().token_type != TokenType::Comma {
            if self.matches(Box::new([TokenType::RightCurlyBracket])) {
                return Value::Object(pairs.into_boxed_slice());
            } else {
                report("Unclosed curly brackets.");
            }
        } else {
            report("Unexpected comma.")
        }

        return Value::Object(Box::new([]));
    }

    fn array(&mut self) -> Value {
        let mut values = Vec::new();
        while self.peek().token_type != TokenType::RightSquareBracket && !self.is_at_end() {
            let value = self.expression();
            values.push(value);
            if self.check(&TokenType::Comma) {
                self.advance();
            }
        }
        if self.previous().token_type != TokenType::Comma {
            if self.matches(Box::new([TokenType::RightSquareBracket])) {
                return Value::Array(values);
            } else {
                report("Unclosed square brackets.");
            }
        } else {
            report("Unexpected comma.")
        }

        return Value::Array(Vec::new());
    }

    fn matches(&mut self, token_types: Box<[TokenType]>) -> bool {
        for token_type in token_types.iter() {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        self.peek().token_type == *token_type
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current = self.current + 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }
}

pub fn run_file(path: &str) {
    let contents = fs::read_to_string(path).expect("Unable to read file");
    let mut lexer = Lexer::new(contents);
    let tokens = lexer.scan_tokens();
    let mut parser = Parser::new(tokens);
    let value = parser.parse();
    println!("{:?}", value);
}

pub fn run_prompt() {
    loop {
        let mut prompt = String::new();
        io::stdin()
            .read_line(&mut prompt)
            .expect("Failed to read line");

        let mut lexer = Lexer::new(prompt.to_owned());
        let tokens = lexer.scan_tokens();
        let mut parser = Parser::new(tokens);
        let expression = parser.parse();
        println!("{:?}", expression);
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
