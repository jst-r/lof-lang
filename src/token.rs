use std::rc::Rc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenKind {
    // One character
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftCurly,
    RightCurly,
    Comma,
    Minus,
    Plus,
    Star,
    Semicolon,
    Slash,
    // One or two characters,
    Dot,
    DotDot,
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // Literals
    Identifier,
    Literal,
    // Keywords
    And,
    Or,
    Class,
    If,
    Else,
    True,
    False,
    Fn,
    For,
    In,
    While,
    Nil,
    Print, // For now, later I want to move it to std
    Return,
    Var,
    Const,
    This,

    Eof,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum LiteralValue {
    None,
    Integer(isize),
    Float(f64),
    String(Rc<str>),
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct TokenStruct {
    pub kind: TokenKind,
    pub lexeme: Rc<str>,
    pub literal: LiteralValue,
    pub line: usize,
    pub id: usize,
}

pub type Token = Rc<TokenStruct>;
