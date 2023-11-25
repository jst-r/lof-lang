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
fn abs(i) {
    if i > 100 {
        return;
    };
    if i < 0 {
        -i
    } else {
        i
    }
}

print abs(101);
print abs(2);
print abs(-3);

fn apply_twice(f, arg) {
    var temp = f(arg);
    f(temp)
}

fn square(n) {
    n * n
}

print apply_twice(square, 2);
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

    dbg!(&prog);

    let prog = prog.into_iter().map(|r| r.unwrap()).collect::<Vec<Stmt>>();

    let mut interpreter = Interpreter::new();

    println!("{:?}", interpreter.interpret(prog));
}
