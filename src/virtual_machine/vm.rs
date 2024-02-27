use std::io::stdout;

use super::{
    chunk::{disassemble_operation, Chunk},
    op_code::OpCode,
    value::Value,
};

pub struct VM {
    pub chunk: Chunk,
    pub instruction_pointer: usize,
}

type InterpretResult = Result<(), ()>;

const DEBUG_TRACE_EXECUTION: bool = true;

impl VM {
    pub fn interpret(&mut self, chunk: Chunk) -> InterpretResult {
        self.chunk = chunk;
        self.instruction_pointer = 0;

        self.run()
    }

    pub fn run(&mut self) -> InterpretResult {
        loop {
            let instruction = self.read_op();

            if DEBUG_TRACE_EXECUTION {
                print!(
                    "{}",
                    disassemble_operation(&self.chunk, self.instruction_pointer - 1)
                );
            }

            match instruction {
                OpCode::Return => return Ok(()),
                OpCode::Constant => {
                    dbg!(self.read_constant());
                }
            }
        }
    }

    fn read_op(&mut self) -> OpCode {
        let instruction = self.chunk.code[self.instruction_pointer]
            .try_into()
            .unwrap();
        self.instruction_pointer += 1;
        instruction
    }

    fn read_constant(&mut self) -> Value {
        let const_index = self.chunk.code[self.instruction_pointer];
        self.instruction_pointer += 1;

        self.chunk.constants[const_index as usize]
    }
}
