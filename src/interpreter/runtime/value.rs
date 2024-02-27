use std::{cell::RefCell, rc::Rc};

use super::{
    callable::Callable,
    class::{Class, Instance},
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
