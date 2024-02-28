use lof_lang::virtual_machine::{chunk::Chunk, op_code::OpCode, vm::VM};

fn main() {
    let mut chunk = Chunk::default();

    let const_offset = chunk.add_constant(42.0);

    chunk.write_operation(OpCode::Constant, [const_offset], 1);
    chunk.write_operation(OpCode::Negate, [], 1);
    chunk.write_operation(OpCode::Return, [], 1);

    let mut vm = VM::new(chunk.clone());

    println!("{:?}", vm.interpret(chunk.clone()));

    // println!("{}", chunk.disassemble().unwrap());
}
