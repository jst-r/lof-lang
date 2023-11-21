use lazy_static::lazy_static;
use std::{collections::BTreeMap, rc::Rc, str::CharIndices};
use thiserror::Error;

use crate::token::{LiteralValue, Token, TokenKind, TokenStruct};

lazy_static! {
    static ref KEYWORDS: BTreeMap<&'static str, TokenKind> = BTreeMap::from([
        ("and", TokenKind::And),
        ("or", TokenKind::Or),
        ("class", TokenKind::Class),
        ("if", TokenKind::If),
        ("else", TokenKind::Else),
        ("true", TokenKind::True),
        ("false", TokenKind::False),
        ("fn", TokenKind::Fn),
        ("for", TokenKind::For),
        ("while", TokenKind::While),
        ("nil", TokenKind::Nil),
        ("print", TokenKind::Print),
        ("return", TokenKind::Return),
        ("var", TokenKind::Var),
        ("const", TokenKind::Const),
    ]);
}

#[derive(Error, Debug, Clone, Copy)]
pub enum ScannerError {
    #[error("unexpected token")]
    UnexpectedToken,
    #[error("unterminated string")]
    UnterminatedString,
}

#[derive(Debug)]
pub struct Scanner<'a> {
    source: &'a str,
    chars: CharIndices<'a>,
    tokens: Vec<Result<Token, ScannerError>>,
    start: usize,
    current: usize,
    line: usize,
}

pub type TokenSlice<'a> = &'a [Result<Token, ScannerError>];

impl<'a> Scanner<'a> {
    pub fn new<S: Into<&'a str>>(source: S) -> Self {
        let source = source.into();
        Scanner {
            source,
            chars: source.char_indices(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 0,
        }
    }

    pub fn reset<S: Into<&'a str>>(&'a mut self, source: S) {
        let source = source.into();

        self.source = source;
        self.chars = source.char_indices();
        self.tokens = vec![];
        self.start = 0;
        self.current = 0;
        self.line = 0;
    }

    pub fn scan_tokens(&'a mut self) -> TokenSlice<'a> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Ok(Rc::new(TokenStruct {
            kind: TokenKind::Eof,
            lexeme: Rc::from("\0"),
            literal: LiteralValue::None,
            line: self.line,
        })));
        self.tokens.as_slice()
    }

    fn scan_token(&mut self) {
        let c = self.advance().unwrap();

        match c {
            '(' => self.add_token(TokenKind::LeftParen),
            ')' => self.add_token(TokenKind::RightParen),
            '[' => self.add_token(TokenKind::LeftBrace),
            ']' => self.add_token(TokenKind::RightBrace),
            '{' => self.add_token(TokenKind::LeftCurly),
            '}' => self.add_token(TokenKind::RightCurly),
            ',' => self.add_token(TokenKind::Comma),
            '.' => self.add_token(TokenKind::Dot),
            '-' => self.add_token(TokenKind::Minus),
            '+' => self.add_token(TokenKind::Plus),
            ';' => self.add_token(TokenKind::Semicolon),
            '*' => self.add_token(TokenKind::Star),
            '!' => {
                if self.matches('=') {
                    self.add_token(TokenKind::BangEqual)
                } else {
                    self.add_token(TokenKind::Bang)
                }
            }
            '=' => {
                if self.matches('=') {
                    self.add_token(TokenKind::EqualEqual)
                } else {
                    self.add_token(TokenKind::Equal)
                }
            }
            '<' => {
                if self.matches('=') {
                    self.add_token(TokenKind::LessEqual)
                } else {
                    self.add_token(TokenKind::Less)
                }
            }
            '>' => {
                if self.matches('=') {
                    self.add_token(TokenKind::GreaterEqual)
                } else {
                    self.add_token(TokenKind::Greater)
                }
            }
            '/' => {
                if self.matches('/') {
                    while self.peek() != Some('\n') && self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenKind::Slash)
                }
            }
            ' ' => {}
            '\r' => {}
            '\t' => {}
            '\n' => self.line += 1,
            '"' => self.string(),

            '0'..='9' => self.number(),
            'a'..='z' | 'A'..='Z' => self.identifier(),

            _ => self.add_error(ScannerError::UnexpectedToken),
        };
    }

    fn advance(&mut self) -> Option<char> {
        match self.chars.next() {
            None => None,
            Some((pos, c)) => {
                self.current = pos + 1;
                Some(c)
            }
        }
    }

    fn peek(&self) -> Option<char> {
        self.chars.clone().next().map(|t| t.1)
    }

    fn peek_next(&self) -> Option<char> {
        let mut temp = self.chars.clone();
        temp.next();
        temp.next().map(|t| t.1)
    }

    fn current_slice(&self) -> &'a str {
        let byte_slice = &self.source.as_bytes()[self.start..self.current];
        std::str::from_utf8(byte_slice).unwrap()
    }

    fn matches(&mut self, expected: char) -> bool {
        match self.peek() {
            None => false,
            Some(c) => {
                if c == expected {
                    self.advance();
                    true
                } else {
                    false
                }
            }
        }
    }

    fn add_token(&mut self, kind: TokenKind) {
        self.add_literal_token(kind, LiteralValue::None)
    }

    fn add_literal_token(&mut self, kind: TokenKind, value: LiteralValue) {
        let lexeme = self.current_slice().into();
        self.tokens.push(Ok(Rc::new(TokenStruct {
            kind,
            lexeme,
            literal: value,
            line: self.line,
        })))
    }

    fn add_error(&mut self, err: ScannerError) {
        self.tokens.push(Err(err));
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn string(&mut self) {
        while self.peek() != Some('"') && !self.is_at_end() {
            self.advance();
        }
        if self.is_at_end() {
            self.add_error(ScannerError::UnterminatedString);
            return;
        }

        // closing "
        self.advance();

        let byte_slice = &self.source.as_bytes()[self.start + 1..self.current - 1];
        let value = std::str::from_utf8(byte_slice).unwrap().into();

        self.add_literal_token(TokenKind::Literal, LiteralValue::String(value));
    }

    fn is_digit(c: char) -> bool {
        c.is_numeric()
    }

    fn is_identifier_char(c: char) -> bool {
        c.is_alphabetic() || c == '_'
    }

    fn consume_digits(&mut self) {
        while self.peek().map_or(false, Scanner::is_digit) {
            self.advance();
        }
    }

    fn number(&mut self) {
        let mut is_float = false;

        self.consume_digits();

        if self.peek() == Some('.') && self.peek_next().map_or(false, Scanner::is_digit) {
            is_float = true;
            self.advance();
            self.consume_digits();
        }

        if is_float {
            self.add_literal_token(
                TokenKind::Literal,
                LiteralValue::Float(self.current_slice().parse::<f64>().unwrap()),
            )
        } else {
            self.add_literal_token(
                TokenKind::Literal,
                LiteralValue::Integer(self.current_slice().parse::<isize>().unwrap()),
            )
        }
    }

    fn identifier(&mut self) {
        while self.peek().map_or(false, Scanner::is_identifier_char) {
            self.advance();
        }

        let text = self.current_slice();
        let token_type = KEYWORDS.get(text);

        match token_type {
            None => self.add_token(TokenKind::Identifier),
            Some(tk) => self.add_token(*tk),
        };
    }
}
