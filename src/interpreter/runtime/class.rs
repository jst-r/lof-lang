use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

use crate::{interpreter::Interpreter, token::Token};

use super::{callable::Callable, function::Function, result::RuntimeResult, value::RuntimeValue};

#[derive(Debug, Clone)]
pub struct Class {
    name: Token,
    methods: BTreeMap<Rc<str>, Function>,
}

impl Class {
    pub fn new(name: Token, methods: BTreeMap<Rc<str>, Function>) -> Self {
        Class { name, methods }
    }

    pub fn find_method(&self, name: &Rc<str>) -> Option<&Function> {
        self.methods.get(name)
    }
}

impl Callable for Class {
    fn call(&self, _: &mut Interpreter, _: Vec<RuntimeValue>) -> RuntimeResult {
        Ok(RuntimeValue::Instance(Rc::new(
            Instance {
                class: Rc::new(self.clone()),
                fields: Default::default(),
            }
            .into(),
        )))
    }

    fn arity(&self) -> usize {
        0
    }
}

#[derive(Debug)]
pub struct Instance {
    class: Rc<Class>,
    fields: BTreeMap<Rc<str>, RuntimeValue>,
}

pub trait InstanceTrait {
    fn get(&self, name: &Token) -> Option<RuntimeValue>;
    fn set(&self, name: &Token, value: RuntimeValue);
}

impl InstanceTrait for Rc<RefCell<Instance>> {
    fn get(&self, name: &Token) -> Option<RuntimeValue> {
        let name = &name.lexeme;

        if let Some(field) = self.borrow().fields.get(name) {
            return field.clone().into();
        }
        if let Some(method) = self.borrow().class.find_method(name) {
            let method = method.bind(self.clone());
            return Some(method.into());
        }

        None
    }

    fn set(&self, name: &Token, value: RuntimeValue) {
        self.borrow_mut().fields.insert(name.lexeme.clone(), value);
    }
}
