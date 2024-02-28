use std::ops::{Add, Div, Mul, Sub};

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
        if DEBUG_TRACE_EXECUTION {
            print!("\n{:=^50}\n", self.chunk.name.unwrap_or(""));
        }

        loop {
            let instruction = self.read_op();

            if DEBUG_TRACE_EXECUTION {
                print!(
                    "{}\n",
                    disassemble_operation(&self.chunk, self.instruction_pointer - 1),
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
                OpCode::Negate => {
                    let val = -self.pop();
                    self.push(val);
                }
                OpCode::Add => self.binary_op(Add::add),
                OpCode::Subtract => self.binary_op(Sub::sub),
                OpCode::Multiply => self.binary_op(Mul::mul),
                OpCode::Divide => self.binary_op(Div::div),
            }

            if DEBUG_TRACE_EXECUTION {
                print!("{:?}\n", &self.stack[0..self.stack_top]);
            }
        }
    }

    fn push(&mut self, value: Value) {
        self.stack[self.stack_top] = value;
        self.stack_top += 1;
    }

    fn pop(&mut self) -> Value {
        self.stack_top -= 1;
        self.stack[self.stack_top]
    }

    fn binary_op(&mut self, op: fn(Value, Value) -> Value) {
        let b = self.pop();
        let a = self.pop();

        self.push(op(a, b))
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

#[cfg(test)]
mod test {
    use crate::virtual_machine::{chunk::Chunk, op_code::OpCode, value::Value, vm::VM};

    #[test]
    fn arithmetic() {
        {
            let mut chunk = Chunk::new_named("Negation");
            let const_offset = chunk.add_constant(42.0);

            chunk.write_operation(OpCode::Constant, [const_offset], 1);
            chunk.write_operation(OpCode::Negate, [], 1);
            chunk.write_operation(OpCode::Return, [], 1);

            let output = VM::new(chunk).run();
            assert_eq!(-42.0, output.unwrap());
        }

        {
            let mut chunk = Chunk::new_named("Addition");
            let a_offset = chunk.add_constant(3.0);
            let b_offset = chunk.add_constant(1.0);
            chunk.write_operation(OpCode::Constant, [a_offset], 1);
            chunk.write_operation(OpCode::Constant, [b_offset], 1);
            chunk.write_operation(OpCode::Add, [], 1);
            chunk.write_operation(OpCode::Return, [], 1);

            let output = VM::new(chunk).run();

            assert_eq!(4.0, output.unwrap());
        }

        fn an_expression(a: Value, b: Value, c: Value, d: Value) {
            let mut chunk = Chunk::new_named("(a + b) * (c - d)");

            let a_offset = chunk.add_constant(a);
            let b_offset = chunk.add_constant(b);
            let c_offset = chunk.add_constant(c);
            let d_offset = chunk.add_constant(d);

            chunk.write_operation(OpCode::Constant, [a_offset], 1);
            chunk.write_operation(OpCode::Constant, [b_offset], 1);
            chunk.write_operation(OpCode::Add, [], 1);

            chunk.write_operation(OpCode::Constant, [c_offset], 1);
            chunk.write_operation(OpCode::Constant, [d_offset], 1);
            chunk.write_operation(OpCode::Subtract, [], 1);

            chunk.write_operation(OpCode::Multiply, [], 1);

            chunk.write_operation(OpCode::Return, [], 1);

            let output = VM::new(chunk).run();

            assert_eq!((a + b) * (c - d), output.unwrap());
        }

        an_expression(1.0, 2.0, 3.0, 5.0);
    }
}
