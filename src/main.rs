mod expression;

mod interpreter;
mod parser;
mod scanner;
mod statement;
mod token;
mod visitor;

use interpreter::resolver::Resolver;
use parser::Parser;
use scanner::Scanner;
use statement::Stmt;

use crate::{interpreter::Interpreter, token::Token};

const SOURCE: &str = r#"
class Test {}

var a = Test();
a.b = 1;
a.b = a.b + 1;

print(a.b);
"#;

// print "fib test";
// fn fib(n) {
//     if n <= 2 {
//         1
//     } else {
//         fib(n - 1) + fib(n - 2)
//     }
// }
// print fib(25);

fn main() {
    run_code(SOURCE);
}

fn run_code(source: &str) {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
    let tokens: Vec<Token> = tokens.iter().map(|t| t.clone().unwrap()).collect();

    let mut parser = Parser::new(tokens.clone());
    let prog = parser.parse();

    let prog = prog.into_iter().map(|r| r.unwrap()).collect::<Vec<Stmt>>();

    let mut resolver = Resolver::default();
    resolver.resolver_pass(&prog);

    for id in resolver.resolutions.keys() {
        println!("{:?}", &tokens.iter().find(|t| t.id == *id));
    }

    // dbg!(&prog);

    dbg!(&resolver.resolutions);

    let mut interpreter = Interpreter::new(resolver);

    interpreter.interpret(prog).unwrap();
}
