use std::rc::Rc;
use thiserror::Error;

use crate::token::Token;

use super::value::RuntimeValue;

#[derive(Error, Debug, PartialEq)]
pub enum RuntimeError {
    #[error("Undefined variable at token {0:?}")]
    UndefinedVariable(Token),
    #[error("Assertion error: {0}")]
    AssertionError(Rc<str>),
    #[error("Type error")]
    TypeError,
}

#[derive(Error, Debug)]
pub enum RuntimeUnwind {
    #[error(transparent)]
    Err(RuntimeError),
    #[error("")]
    Return(RuntimeValue),
}

pub type RuntimeResult = Result<RuntimeValue, RuntimeUnwind>;
pub type RuntimeResultNoValue = Result<(), RuntimeUnwind>;

impl From<RuntimeValue> for RuntimeResult {
    fn from(val: RuntimeValue) -> Self {
        Ok(val)
    }
}

impl From<RuntimeError> for RuntimeResult {
    fn from(val: RuntimeError) -> Self {
        Err(RuntimeUnwind::Err(val))
    }
}

impl From<RuntimeError> for RuntimeUnwind {
    fn from(val: RuntimeError) -> Self {
        RuntimeUnwind::Err(val)
    }
}
