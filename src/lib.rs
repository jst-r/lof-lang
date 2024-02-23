use expression::{BoxExpr, ExprVisitor};
use interpreter::runtime_value::{RuntimeUnwind, RuntimeValue};
use scanner::ScannerError;
use thiserror::Error;

use parser::ParserError;

use crate::{
    interpreter::{resolver::Resolver, Interpreter},
    parser::Parser,
    scanner::Scanner,
    statement::Stmt,
    token::Token,
};

pub mod expression;
pub mod interpreter;
pub mod parser;
pub mod scanner;
pub mod statement;
pub mod token;
pub mod visitor;

pub fn parse_program(source: &str) -> Result<Vec<Stmt>, LofError> {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
    let tokens: Vec<Token> = {
        let tok_result: Result<Vec<Token>, ScannerError> =
            tokens.iter().cloned().collect();
        tok_result?
    };

    let mut parser = Parser::new(tokens.clone());
    let prog: Result<Vec<Stmt>, ParserError> = parser.parse().into_iter().collect();

    Ok(prog?)
}

pub fn parse_expression(source: &str) -> Result<BoxExpr, LofError> {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
    let tokens: Vec<Token> = {
        let tok_result: Result<Vec<Token>, ScannerError> =
            tokens.iter().cloned().collect();
        tok_result?
    };

    let mut parser = Parser::new(tokens.clone());

    Ok(parser.expression()?)
}

pub fn eval_expr(expr: BoxExpr) -> Result<RuntimeValue, LofError> {
    let mut interpreter = Interpreter::new(Resolver::default());
    Ok(interpreter.visit(&expr)?)
}

pub fn run_expr(source: &str) -> Result<RuntimeValue, LofError> {
    eval_expr(parse_expression(source)?)
}

pub fn run_code(source: &str) -> Result<(), LofError> {
    let prog = parse_program(source)?;

    let mut resolver = Resolver::default();
    resolver.resolver_pass(&prog);

    // dbg!(&prog);

    // dbg!(&resolver.resolutions);

    let mut interpreter = Interpreter::new(resolver);

    interpreter.interpret(prog).unwrap();

    Ok(())
}

#[derive(Error, Debug)]
pub enum LofError {
    #[error("Scanner error")]
    ScannerError(#[from] ScannerError),
    #[error("Parser error")]
    ParserError(#[from] ParserError),
    #[error("RuntimeError")]
    RuntimeError(#[from] RuntimeUnwind),
}
