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
    fn call(&self, interpreter: &mut Interpreter, args: Vec<RuntimeValue>) -> RuntimeResult {
        let instance: Rc<RefCell<Instance>> = Rc::new(
            Instance {
                class: Rc::new(self.clone()),
                fields: Default::default(),
            }
            .into(),
        );

        if let Some(init) = self.find_method(&"init".into()) {
            init.bind(instance.clone()).call(interpreter, args).unwrap();
        }

        Ok(RuntimeValue::Instance(instance))
    }

    fn arity(&self) -> usize {
        if let Some(init) = self.find_method(&"init".into()) {
            init.arity()
        } else {
            0
        }
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
