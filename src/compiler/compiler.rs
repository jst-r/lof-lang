use num_enum::{IntoPrimitive, TryFromPrimitive};

use crate::virtual_machine::{chunk::Chunk, op_code::OpCode, value::Value};

use super::{
    scanner::{Scanner, ScannerError},
    token::{Token, TokenKind},
};

struct Compiler<'source> {
    compiling_chunk: Chunk,
    scanner: Scanner<'source>,
}

#[repr(u8)]
#[derive(IntoPrimitive, TryFromPrimitive, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
    None,
    Assignment, // =
    Or,         // or
    And,        // and
    Equality,   // == !=
    Comparison, // < > <= >=
    Term,       // + -
    Factor,     // * /
    Unary,      // ! -
    Call,       // () .
    Primary,
}

impl Precedence {
    fn next(self) -> Self {
        (u8::from(self) + 1u8).try_into().unwrap()
    }
}

type ParseFn<'source> = fn(&mut Compiler<'source>) -> Result<(), CompilerError>;
struct ParseRule<'source> {
    prefix: Option<ParseFn<'source>>,
    infix: Option<ParseFn<'source>>,
    precedence: Precedence,
}

impl<'source> ParseRule<'source> {
    pub fn from_token(token: &Token) -> Self {
        Self::from_kind(token.kind)
    }

    pub fn from_kind(kind: TokenKind) -> Self {
        let (prefix, infix, precedence): (
            Option<ParseFn<'source>>,
            Option<ParseFn<'source>>,
            Precedence,
        ) = match kind {
            TokenKind::LeftParen => (Some(Compiler::grouping), None, Precedence::None),

            TokenKind::Minus => (
                Some(Compiler::unary),
                Some(Compiler::binary),
                Precedence::None,
            ),
            TokenKind::Plus => (None, Some(Compiler::binary), Precedence::Term),
            TokenKind::Slash => (None, Some(Compiler::binary), Precedence::Factor),
            TokenKind::Star => (None, Some(Compiler::binary), Precedence::Factor),

            TokenKind::Literal => (Some(Compiler::number), None, Precedence::None),
            _ => (None, None, Precedence::None),
        };

        Self {
            prefix,
            infix,
            precedence,
        }
    }
}

#[allow(dead_code)]
impl<'source> Compiler<'source> {
    pub fn new(scanner: Scanner<'source>) -> Self {
        Self {
            compiling_chunk: Chunk::new(),
            scanner,
        }
    }

    pub fn compile(&mut self) -> Result<(), CompilerError> {
        self.expression()?;
        self.consume(TokenKind::Eof);

        Ok(())
    }

    fn consume(&mut self, token: TokenKind) {
        todo!()
    }

    fn parse_precedence(&mut self, precedence: Precedence) -> Result<(), CompilerError> {
        let lhs = self.scanner.next()?;

        ParseRule::from_token(&lhs)
            .prefix
            .ok_or(CompilerError::ExpectedExpression)?(self)?;

        loop {
            let infix_rule = ParseRule::from_token(&self.previous).infix.unwrap();
            infix_rule(self)?;
        }

        Ok(())
    }

    fn expression(&mut self) -> Result<(), CompilerError> {
        self.parse_precedence(Precedence::Assignment)
    }

    fn grouping(&mut self) -> Result<(), CompilerError> {
        self.expression()?;
        self.consume(TokenKind::RightParen);

        Ok(())
    }

    fn binary(&mut self) -> Result<(), CompilerError> {
        let operator_kind = self.previous.kind;

        let rule = ParseRule::from_kind(operator_kind);

        self.parse_precedence(rule.precedence.next())?;

        match operator_kind {
            TokenKind::Plus => self.emit_op_code(OpCode::Add),
            TokenKind::Minus => self.emit_op_code(OpCode::Subtract),
            TokenKind::Star => self.emit_op_code(OpCode::Multiply),
            TokenKind::Slash => self.emit_op_code(OpCode::Divide),

            _ => Err(CompilerError::InvalidOperator)?,
        }

        Ok(())
    }

    fn unary(&mut self) -> Result<(), CompilerError> {
        let operator_kind = self.previous.kind;

        self.parse_precedence(Precedence::Unary)?;

        match operator_kind {
            TokenKind::Minus => self.emit_op_code(OpCode::Negate),
            _ => panic!("invalid unary"),
        };

        Ok(())
    }

    fn number(&mut self) -> Result<(), CompilerError> {
        let value = self.previous.lexeme.parse().unwrap();
        self.emit_constant(value);

        Ok(())
    }

    fn error_at_current(&self, err: ScannerError) {
        todo!()
    }

    fn emit_op_code(&mut self, op: OpCode) {
        self.compiling_chunk.write_op_code(op, self.previous.line)
    }

    fn emit_operand(&mut self, operand: u8) {
        self.compiling_chunk
            .write_operand(operand, self.previous.line)
    }

    fn emit_op_code_operand(&mut self, op: OpCode, operand: u8) {
        self.emit_op_code(op);
        self.emit_operand(operand);
    }

    fn emit_constant(&mut self, value: Value) {
        let const_index = self.make_constant(value);
        self.emit_op_code_operand(OpCode::Constant, const_index);
    }

    fn make_constant(&mut self, value: f64) -> u8 {
        self.compiling_chunk.add_constant(value)
    }
}

enum CompilerError {
    ExpectedExpression,
    InvalidOperator,
    ScannerError(ScannerError),
}

impl From<ScannerError> for CompilerError {
    fn from(value: ScannerError) -> Self {
        Self::ScannerError(value)
    }
}
