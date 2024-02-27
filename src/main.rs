use lof_lang::virtual_machine::{chunk::Chunk, op_code::OpCode, vm::VM};

fn main() {
    let mut chunk = Chunk::default();

    let const_offset = chunk.add_constant(42.0);
    chunk.write_op_code(OpCode::Constant, 1);
    chunk.write_operand(const_offset, 1);
    chunk.write_op_code(OpCode::Return, 1);

    let mut vm = VM::new(chunk.clone());

    println!("{:?}", vm.interpret(chunk.clone()));

    // println!("{}", chunk.disassemble().unwrap());
}
