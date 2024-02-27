use std::{default, io::stdout};

use super::{
    chunk::{disassemble_operation, Chunk},
    op_code::OpCode,
    value::Value,
};

const DEBUG_TRACE_EXECUTION: bool = true;
const STACK_MAX: usize = 256;

pub struct VM {
    pub chunk: Chunk,
    pub instruction_pointer: usize,
    pub stack: [Value; STACK_MAX],
    pub stack_top: usize,
}

type InterpretResult = Result<Value, ()>;

impl VM {
    pub fn new(chunk: Chunk) -> Self {
        Self {
            chunk,
            instruction_pointer: 0,
            stack: [0.0; STACK_MAX],
            stack_top: 0,
        }
    }

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
                    "{}\nstack: {:?}\n",
                    disassemble_operation(&self.chunk, self.instruction_pointer - 1),
                    &self.stack[0..self.stack_top]
                );
            }

            match instruction {
                OpCode::Return => {
                    let val = self.pop();
                    return Ok(val);
                }
                OpCode::Constant => {
                    let constant = self.read_constant();
                    self.push(constant);
                }
            }
        }
    }

    fn push(&mut self, value: Value) {
        self.stack[self.stack_top];
        self.stack_top += 1;
    }

    fn pop(&mut self) -> Value {
        self.stack_top -= 1;
        self.stack[self.stack_top]
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
