use lazy_static::lazy_static;
use std::{collections::BTreeMap, str::CharIndices};
use thiserror::Error;

use crate::compiler::token::{LiteralValue, Token, TokenKind};

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
        ("in", TokenKind::In),
        ("while", TokenKind::While),
        ("nil", TokenKind::Nil),
        ("print", TokenKind::Print),
        ("return", TokenKind::Return),
        ("var", TokenKind::Var),
        ("const", TokenKind::Const),
        ("this", TokenKind::This)
    ]);
}

#[derive(Error, Debug, Clone, Copy, PartialEq)]
pub enum ScannerError {
    #[error("unexpected token")]
    UnexpectedToken,
    #[error("unterminated string")]
    UnterminatedString,
}

#[derive(Debug)]
pub struct Scanner<'source> {
    source: &'source str,
    chars: CharIndices<'source>,
    tokens: Vec<Result<Token<'source>, ScannerError>>,
    start: usize,
    current: usize,
    line: usize,
    current_id: usize,
}

pub type ScannerResult<'source> = Result<Token<'source>, ScannerError>;

impl<'source> Scanner<'source> {
    pub fn new<S: Into<&'source str>>(source: S) -> Self {
        let source = source.into();
        let mut scanner = Scanner {
            source,
            chars: source.char_indices(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 0,
            current_id: 0,
        };

        scanner.scan_tokens();

        scanner.tokens.reverse();

        scanner
    }

    pub fn next(&mut self) -> ScannerResult<'source> {
        self.tokens.pop().unwrap_or(self.make_eof())
    }

    pub fn peek(&self) -> ScannerResult<'source> {
        self.tokens.last().cloned().unwrap_or(self.make_eof())
    }

    fn make_eof(&self) -> ScannerResult<'static> {
        Ok(Token {
            kind: TokenKind::Eof,
            lexeme: "\0",
            literal: LiteralValue::None,
            line: self.line,
            id: self.current_id,
        })
    }

    fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.current_id += 1;
        let eof = self.make_eof();
        self.tokens.push(eof);
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
            '-' => self.add_token(TokenKind::Minus),
            '+' => self.add_token(TokenKind::Plus),
            ';' => self.add_token(TokenKind::Semicolon),
            '*' => self.add_token(TokenKind::Star),
            '!' => self.add_token_lookahead('=', TokenKind::BangEqual, TokenKind::Bang),
            '.' => self.add_token_lookahead('.', TokenKind::DotDot, TokenKind::Dot),
            '=' => self.add_token_lookahead('=', TokenKind::EqualEqual, TokenKind::Equal),
            '<' => self.add_token_lookahead('=', TokenKind::LessEqual, TokenKind::Less),
            '>' => self.add_token_lookahead('=', TokenKind::GreaterEqual, TokenKind::Greater),
            '/' => {
                if self.matches('/') {
                    while self.peek_char() != Some('\n') && !self.is_at_end() {
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
            'a'..='z' | 'A'..='Z' | '_' => self.identifier(),

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

    fn peek_char(&self) -> Option<char> {
        self.chars.clone().next().map(|t| t.1)
    }

    fn peek_next(&self) -> Option<char> {
        let mut temp = self.chars.clone();
        temp.next();
        temp.next().map(|t| t.1)
    }

    fn current_slice(&self) -> &'source str {
        let byte_slice = &self.source.as_bytes()[self.start..self.current];
        std::str::from_utf8(byte_slice).unwrap()
    }

    fn matches(&mut self, expected: char) -> bool {
        match self.peek_char() {
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

    fn add_token_lookahead(&mut self, next_kind: char, two: TokenKind, one: TokenKind) {
        if self.matches(next_kind) {
            self.add_token(two)
        } else {
            self.add_token(one)
        }
    }

    fn add_literal_token(&mut self, kind: TokenKind, value: LiteralValue) {
        let lexeme = self.current_slice().into();
        self.current_id += 1;
        self.tokens.push(Ok(Token {
            kind,
            lexeme,
            literal: value,
            line: self.line,
            id: self.current_id,
        }))
    }

    fn add_error(&mut self, err: ScannerError) {
        self.tokens.push(Err(err));
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn string(&mut self) {
        while self.peek_char() != Some('"') && !self.is_at_end() {
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
        c.is_digit(10)
    }

    fn is_identifier_char(c: char) -> bool {
        c.is_alphabetic() || c == '_'
    }

    fn consume_digits(&mut self) {
        while self.peek_char().map_or(false, Scanner::is_digit) {
            self.advance();
        }
    }

    fn number(&mut self) {
        let mut is_float = false;

        self.consume_digits();

        if self.peek_char() == Some('.') && self.peek_next().map_or(false, Scanner::is_digit) {
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
        while self.peek_char().map_or(false, |c| {
            Scanner::is_identifier_char(c) || Scanner::is_digit(c)
        }) {
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
