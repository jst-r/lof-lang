use num_enum::{FromPrimitive, IntoPrimitive, TryFromPrimitive};

use crate::virtual_machine::{chunk::Chunk, op_code::OpCode, value::Value};

use super::{
    scanner::{Scanner, ScannerError, ScannerResult},
    token::{Token, TokenKind},
};

struct Compiler<'source> {
    current: Token<'source>,
    previous: Token<'source>,
    compiling_chunk: Chunk,
    scanner: Scanner<'source>,
}

#[repr(u8)]
#[derive(IntoPrimitive, TryFromPrimitive)]
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

impl<'source> Compiler<'source> {
    pub fn new() -> Self {
        todo!()
    }

    pub fn compile(&mut self, source: &'source str) {
        self.advance();
        self.expression();
        self.consume(TokenKind::Eof);
    }

    fn advance(&mut self) {
        self.previous = self.current.clone();

        loop {
            let next = self.scanner.next();
            match next {
                Ok(current) => {
                    self.current = current;
                    break;
                }
                Err(err) => {
                    self.error_at_current(err);
                }
            }
        }
    }

    fn consume(&mut self, token: TokenKind) {
        todo!()
    }

    fn parse_precedence(&mut self, precedence: Precedence) {}

    fn expression(&mut self) {
        self.parse_precedence(Precedence::Assignment)
    }

    fn grouping(&mut self) {
        self.expression();
        self.consume(TokenKind::RightParen);
    }

    fn binary(&mut self) {
        let operator_kind = self.previous.kind;
    }

    fn unary(&mut self) {
        let operator_kind = self.previous.kind;

        self.parse_precedence(Precedence::Unary);

        match operator_kind {
            TokenKind::Minus => self.emit_op_code(OpCode::Negate),
            _ => panic!("invalid unary"),
        }
    }

    fn number(&mut self) {
        let value = self.previous.lexeme.parse().unwrap();
        self.emit_constant(value);
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
