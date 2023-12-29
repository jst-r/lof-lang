use std::rc::Rc;
use thiserror::Error;

use super::runtime_type::{self, Class};

#[derive(Debug, Clone)]
pub enum RuntimeValue {
    String(Rc<str>), //TODO: this pushes the enum size to 24 bytes, which is not ideal
    Integer(isize),
    Float(f64),
    Bool(bool),
    Function(Rc<dyn runtime_type::Callable>), // Functions are boxed to avoid bloating the enum size
    Class(Rc<Class>),
    // those are to be removed when structs and enums are implemented
    Range(isize, isize),
    Unit,
}

#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error("Undefined variable")]
    UndefinedVariable,
}

#[derive(Debug)]
pub enum RuntimeUnwind {
    Err(RuntimeError),
    Return(RuntimeValue),
}

pub type RuntimeResult = Result<RuntimeValue, RuntimeUnwind>;
pub type RuntimeResultNoValue = Result<(), RuntimeUnwind>;

impl Into<RuntimeResult> for RuntimeValue {
    fn into(self) -> RuntimeResult {
        Ok(self)
    }
}

impl Into<RuntimeResult> for RuntimeError {
    fn into(self) -> RuntimeResult {
        Err(RuntimeUnwind::Err(self))
    }
}

impl Into<RuntimeUnwind> for RuntimeError {
    fn into(self) -> RuntimeUnwind {
        RuntimeUnwind::Err(self)
    }
}
