use lof_lang::{
    compiler::{scanner::Scanner, token::TokenKind},
    virtual_machine::{chunk::Chunk, op_code::OpCode, vm::VM},
};

fn main() {
    let source = "1 + 2 * 3";

    let mut scanner = Scanner::new(source);

    loop {
        let token = scanner.next().unwrap();

        dbg!(&token);

        if token.kind == TokenKind::Eof {
            break;
        }
    }
}
