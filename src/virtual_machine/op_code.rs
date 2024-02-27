use std::fmt::Display;

use num_enum::{IntoPrimitive, TryFromPrimitive};

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
pub enum OpCode {
    Return,
    Constant,
}

impl OpCode {
    pub fn num_operands(&self) -> usize {
        match *self {
            OpCode::Return => 0,
            OpCode::Constant => 1,
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
            }
        )
    }
}
