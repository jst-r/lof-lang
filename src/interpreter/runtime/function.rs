use std::cell::RefCell;
use std::iter::zip;
use std::{fmt::Debug, rc::Rc};

use crate::{
    expression::BoxExpr,
    interpreter::{
        environment::{EnvironmentTrait, WrappedEnv},
        Interpreter,
    },
    token::Token,
    visitor::AcceptMut,
};

use super::class::Instance;
use super::{
    callable::Callable,
    result::{RuntimeResult, RuntimeUnwind},
    value::RuntimeValue,
};

#[derive(Debug, Clone)]
pub struct Function {
    pub name: Token,
    pub closure: WrappedEnv,
    pub args: Vec<Token>,
    pub body: BoxExpr,
}

impl Function {
    pub fn bind(&self, instance: Rc<RefCell<Instance>>) -> Self {
        let mut env = self.closure.create_child();
        env.define("this".into(), RuntimeValue::Instance(instance));

        Function {
            name: self.name.clone(),
            closure: env,
            args: self.args.clone(),
            body: self.body.clone(),
        }
    }
}

impl Callable for Function {
    fn call(&self, interpreter: &mut Interpreter, args: Vec<RuntimeValue>) -> RuntimeResult {
        let prev_env = interpreter.environment.clone();
        interpreter.environment = self.closure.create_child();

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

        interpreter.environment = prev_env;

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
