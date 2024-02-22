use std::{cell::RefCell, rc::Rc};
use thiserror::Error;

use super::runtime_type::{Callable, Class, Instance};

#[derive(Debug, Clone)]
pub enum RuntimeValue {
    String(Rc<str>), //TODO: this pushes the enum size to 24 bytes, which is not ideal
    Integer(isize),
    Float(f64),
    Bool(bool),
    Function(Rc<dyn Callable>), // Functions are boxed to avoid bloating the enum size
    Class(Rc<Class>),
    Instance(Rc<RefCell<Instance>>),
    // those are to be removed when structs and enums are implemented
    Range(isize, isize),
    Unit,
}

#[derive(Error, Debug, PartialEq)]
pub enum RuntimeError {
    #[error("Undefined variable")]
    UndefinedVariable,
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
