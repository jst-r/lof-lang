pub mod value;
pub mod vm;

use self::value::Value;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::error::Error;
use std::fmt::Display;
use std::io::BufWriter;
use std::io::Write;

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
pub enum OpCode {
    Return,
    Constant,
}

#[derive(Default)]
pub struct Chunk {
    code: Vec<u8>,
    lines: Vec<usize>,
    constants: Vec<Value>,
}

impl Chunk {
    pub fn write_op_code(&mut self, op: OpCode, line_number: usize) {
        self.code.push(op.into());
        self.lines.push(line_number);
    }

    pub fn write_operand(&mut self, operand: u8, line_number: usize) {
        self.code.push(operand);
        self.lines.push(line_number);
    }

    pub fn add_constant(&mut self, val: Value) -> u8 {
        self.constants.push(val);
        (self.constants.len() - 1) as u8
    }

    pub fn disassemble(&self) -> Result<String, Box<dyn Error>> {
        let mut buffer = BufWriter::new(Vec::new());

        let mut offset = 0;

        while offset < self.code.len() {
            offset = disassemble_operation(self, offset, &mut buffer)?;
        }
        Ok(String::from_utf8(buffer.into_inner()?)?)
    }
}

impl OpCode {
    fn num_operands(&self) -> usize {
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

fn disassemble_operation(
    chunk: &Chunk,
    offset: usize,
    buffer: &mut BufWriter<Vec<u8>>,
) -> Result<usize, Box<dyn Error>> {
    let code = chunk.code.as_slice();

    write!(buffer, "{:>4}", offset)?;

    if offset > 0 && chunk.lines[offset] == chunk.lines[offset - 1] {
        write!(buffer, "     |")?;
    } else {
        write!(buffer, " {:>5}", chunk.lines[offset])?;
    }

    let op: OpCode = code[offset].try_into()?;
    write!(buffer, "\t{}", op)?;
    let n_operands = op.num_operands();
    for i in 1..=n_operands {
        write!(buffer, " {}", code[offset + i])?;
    }

    write!(buffer, "\t")?;
    for i in 1..=n_operands {
        write!(
            buffer,
            "[{}]: {}; ",
            code[offset + i],
            chunk.constants[code[offset + i] as usize]
        )?;
    }

    write!(buffer, "\n")?;

    Ok(offset + 1 + n_operands)
}
