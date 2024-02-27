use std::{cell::RefCell, rc::Rc};

use super::{
    callable::Callable,
    class::{Class, Instance},
    function::Function,
};

#[derive(Debug, Clone)]
pub enum RuntimeValue {
    String(Rc<str>), //TODO: this pushes the enum size to 24 bytes, which is not ideal
    Integer(isize),
    Float(f64),
    Bool(bool),
    Function(Rc<dyn Callable>),
    Class(Rc<Class>),
    Instance(Rc<RefCell<Instance>>),
    // those are to be removed when structs and enums are implemented
    Range(isize, isize),
    Unit,
}

impl From<Function> for RuntimeValue {
    fn from(value: Function) -> Self {
        Self::Function(Rc::new(value))
    }
}

impl From<Class> for RuntimeValue {
    fn from(value: Class) -> Self {
        Self::Class(Rc::new(value))
    }
}
