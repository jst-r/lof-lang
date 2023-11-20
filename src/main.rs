mod expression;

mod interpreter;
mod parser;
mod scanner;
mod statement;
mod token;
mod visitor;

use std::rc::Rc;

use parser::Parser;
use scanner::Scanner;
use statement::Stmt;

use crate::{interpreter::Interpreter, token::Token};

const SOURCE: &'static str = r#"
var a=1;
var b=2;
var c=3;
{
    var a = "inner a";
    print "inner scope";
    print a;
    print b;
    print c;

};
print "outer scope";
print a;
print b;
print c;
"#;

fn main() {
    run_code(SOURCE);
}

fn run_repl() {
    let mut interpreter = Interpreter::default();
    loop {
        let mut source = String::new();
        std::io::stdin().read_line(&mut source).unwrap();

        let source = Rc::from(source);

        let mut scanner = Scanner::new(&*source);
        let tokens = scanner.scan_tokens();
        let tokens: Vec<Token> = tokens.iter().map(|t| t.clone().unwrap()).collect();
        let mut parser = Parser::new(tokens);
        let mut prog = parser.parse();

        if prog.len() != 1 {
            println!("One statement at a time, please");
            continue;
        }

        let prog = prog.pop().unwrap();

        match prog {
            Ok(stmt) => interpreter.interpret(vec![stmt]),
            Err(err) => println!("{}", err),
        }
    }
}

fn run_code(source: &str) {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
    let tokens: Vec<Token> = tokens.iter().map(|t| t.clone().unwrap()).collect();
    let mut parser = Parser::new(tokens);
    let prog = parser.parse();

    let prog = prog.into_iter().map(|r| r.unwrap()).collect::<Vec<Stmt>>();

    let mut interpreter = Interpreter::default();

    interpreter.interpret(prog);
}
