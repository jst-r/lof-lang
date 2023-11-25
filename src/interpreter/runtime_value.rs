use std::rc::Rc;

use super::runtime_type;

#[derive(Debug, Clone)]
pub enum RuntimeValue {
    String(Rc<str>), //TODO: this pushes the enum size to 24 bytes, which is not ideal
    Integer(isize),
    Float(f64),
    Bool(bool),
    Function(Rc<dyn runtime_type::Callable>), // Functions are boxed to avoid bloating the enum size
    // those are to be removed when structs and enums are implemented
    Range(isize, isize),
    Unit,
}
