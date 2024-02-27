use std::{collections::BTreeMap, rc::Rc};

use crate::{interpreter::Interpreter, token::Token};

use super::{callable::Callable, result::RuntimeResult, value::RuntimeValue};

#[derive(Debug, Clone)]
pub struct Class {
    name: Token,
    methods: BTreeMap<Rc<str>, RuntimeValue>,
}

impl Class {
    pub fn new(name: Token, methods: BTreeMap<Rc<str>, RuntimeValue>) -> Self {
        Class { name, methods }
    }

    pub fn find_method(&self, name: &Rc<str>) -> Option<&RuntimeValue> {
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

impl Instance {
    pub fn get(&self, name: &Token) -> Option<RuntimeValue> {
        let name = &name.lexeme;

        if let Some(field) = self.fields.get(name) {
            return field.clone().into();
        }
        if let Some(method) = self.class.find_method(name) {
            return method.clone().into();
        }

        None
    }

    pub fn set(&mut self, name: &Token, value: RuntimeValue) {
        self.fields.insert(name.lexeme.clone(), value);
    }
}
