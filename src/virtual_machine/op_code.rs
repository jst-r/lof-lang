use std::fmt::Display;

use num_enum::{IntoPrimitive, TryFromPrimitive};

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
pub enum OpCode {
    Return,
    Constant,
    Negate,
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl OpCode {
    pub const fn num_operands(&self) -> usize {
        match *self {
            OpCode::Constant => 1,
            _ => 0,
        }
    }
}

impl Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                OpCode::Return => "RETURN",
                OpCode::Constant => "CONSTANT",
                OpCode::Negate => "NEGATE",
                OpCode::Add => "ADD",
                OpCode::Subtract => "SUBTRACT",
                OpCode::Multiply => "MULTIPLY",
                OpCode::Divide => "DIVIDE",
            }
        )
    }
}
