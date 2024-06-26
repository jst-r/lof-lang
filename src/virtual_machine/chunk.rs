use std::error::Error;
use std::io::BufWriter;
use std::io::Write;

use super::op_code::OpCode;
use super::value::Value;

#[derive(Default, Clone, Debug)]
pub struct Chunk {
    pub code: Vec<u8>,
    pub lines: Vec<usize>,
    pub constants: Vec<Value>,
    pub name: Option<&'static str>,
}

impl Chunk {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_named(name: &'static str) -> Self {
        Self {
            name: Some(name),
            ..Default::default()
        }
    }

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

    pub fn write_operation<const NUM_OPERANDS: usize>(
        &mut self,
        op: OpCode,
        operands: [u8; NUM_OPERANDS],
        line_number: usize,
    ) {
        assert!(NUM_OPERANDS == op.num_operands());
        self.write_op_code(op, line_number);

        match op {
            OpCode::Constant => self.write_operand(operands[0], line_number),
            _ => {}
        }
    }

    pub fn disassemble(&self) -> Result<String, Box<dyn Error>> {
        let mut buffer = BufWriter::new(Vec::new());

        let mut offset = 0;

        write!(&mut buffer, "{:=^50}", self.name.unwrap_or(""))?;

        while offset < self.code.len() {
            offset = disassemble_operation_write(self, offset, &mut buffer)?;
        }
        Ok(String::from_utf8(buffer.into_inner()?)?)
    }
}

pub fn disassemble_operation_write(
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

    Ok(offset + 1 + n_operands)
}

pub fn disassemble_operation(chunk: &Chunk, offset: usize) -> String {
    let mut buffer = BufWriter::new(Vec::new());
    disassemble_operation_write(chunk, offset, &mut buffer).unwrap();
    String::from_utf8(buffer.into_inner().unwrap()).unwrap()
}
