use std::{fmt::Debug, iter::zip};

use crate::{expression::BoxExpr, token::Token, visitor::AcceptMut};

use super::{
    runtime_value::{RuntimeResult, RuntimeUnwind, RuntimeValue},
    Interpreter,
};

// Considering making all runtime values a struct with a marker trait. But that seems like a pain

pub trait Callable: Debug {
    fn call(&self, interpreter: &mut Interpreter, args: Vec<RuntimeValue>) -> RuntimeResult;
    fn arity(&self) -> usize;
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: Token,
    pub args: Vec<Token>,
    pub body: BoxExpr,
}

impl Callable for Function {
    fn call(&self, interpreter: &mut Interpreter, args: Vec<RuntimeValue>) -> RuntimeResult {
        interpreter.environment.push();

        for (arg_token, arg_val) in zip(self.args.iter(), args) {
            interpreter
                .environment
                .define(arg_token.lexeme.clone(), arg_val);
        }

        let return_value = (&self.body).accept(interpreter);

        let return_value = match return_value {
            Err(RuntimeUnwind::Return(val)) => Ok(val),
            _ => return_value,
        };

        interpreter.environment.pop();

        return_value
    }

    fn arity(&self) -> usize {
        self.args.len()
    }
}

// kinda proud of this one
pub struct NativeFunctionWrapper<const N: usize, F: Fn([RuntimeValue; N]) -> RuntimeResult> {
    pub function: F,
}

impl<const N: usize, F: Fn([RuntimeValue; N]) -> RuntimeResult> Callable
    for NativeFunctionWrapper<N, F>
{
    fn call(&self, _: &mut Interpreter, args: Vec<RuntimeValue>) -> RuntimeResult {
        (self.function)(args.try_into().expect("invalid number of arguments"))
    }

    fn arity(&self) -> usize {
        N
    }
}

impl<const N: usize, F: Fn([RuntimeValue; N]) -> RuntimeResult> Debug
    for NativeFunctionWrapper<N, F>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NativeFunctionWrapper")
            .field("function", &"<native function>")
            .field("arity", &N)
            .finish()
    }
}
