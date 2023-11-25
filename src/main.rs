mod expression;

mod interpreter;
mod parser;
mod scanner;
mod statement;
mod token;
mod visitor;

use parser::Parser;
use scanner::Scanner;
use statement::Stmt;

use crate::{interpreter::Interpreter, token::Token};

const SOURCE: &str = r#"
fn make_counter() {
    var i = 0;

    fn count() {
        print i;
        i = i + 1;
    }

    return count;
}

var count = make_counter();

for k in 0..100 {
    count();
};
"#;

fn main() {
    run_code(SOURCE);
}

fn run_code(source: &str) {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
    let tokens: Vec<Token> = tokens.iter().map(|t| t.clone().unwrap()).collect();
    let mut parser = Parser::new(tokens);
    let prog = parser.parse();

    let prog = prog.into_iter().map(|r| r.unwrap()).collect::<Vec<Stmt>>();

    let mut interpreter = Interpreter::new();

    interpreter.interpret(prog).unwrap();
}
