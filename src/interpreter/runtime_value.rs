use std::rc::Rc;
use thiserror::Error;

use super::runtime_type::{self, Class, Instance};

#[derive(Debug, Clone)]
pub enum RuntimeValue {
    String(Rc<str>), //TODO: this pushes the enum size to 24 bytes, which is not ideal
    Integer(isize),
    Float(f64),
    Bool(bool),
    Function(Rc<dyn runtime_type::Callable>), // Functions are boxed to avoid bloating the enum size
    Class(Rc<Class>),
    Instance(Rc<Instance>),
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
