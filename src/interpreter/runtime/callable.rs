use std::fmt::Debug;

use crate::interpreter::Interpreter;

use super::{result::RuntimeResult, value::RuntimeValue};

pub trait Callable: Debug {
    fn call(&self, interpreter: &mut Interpreter, args: Vec<RuntimeValue>) -> RuntimeResult;
    fn arity(&self) -> usize;
}
