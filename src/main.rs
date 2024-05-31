use lof_lang::{
    compiler::{
        scanner::{Scanner, ScannerError},
        token::TokenKind,
    },
    virtual_machine::{chunk::Chunk, op_code::OpCode, vm::VM},
};

fn main() {
    let source = "1 + 2 * 3";

    let mut scanner = Scanner::new(source);

    loop {
        let token = scanner.next();

        dbg!(&token);

        if let Err(ScannerError::Eof) = token {
            break;
        }
    }
}
