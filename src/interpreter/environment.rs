use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

use super::runtime_value::{RuntimeResult, RuntimeValue};

use crate::token::Token;

pub type WrappedEnv = Rc<RefCell<Environment>>;

#[derive(Debug, Default)]
pub struct Environment {
    pub enclosing: Option<WrappedEnv>,
    pub values: BTreeMap<Rc<str>, RuntimeValue>,
}

pub trait EnvironmentTrait {
    fn new(enclosing: Option<&Self>) -> Self;

    fn create_child(&self) -> Self;
    fn define(&mut self, name: Rc<str>, value: RuntimeValue);
    fn get(&self, name: &Token) -> Option<RuntimeValue>;
    fn assign(&mut self, name: &Token, value: RuntimeValue) -> RuntimeResult;
    fn get_at(&self, name: &Token, distance: usize) -> RuntimeValue;
    fn assign_at(&self, name: &Token, value: RuntimeValue, distance: usize);
    fn ancestor(&self, distance: usize) -> Self;
}

impl EnvironmentTrait for WrappedEnv {
    fn new(enclosing: Option<&Self>) -> Self {
        Rc::new(RefCell::new(Environment {
            enclosing: enclosing.cloned(),
            values: BTreeMap::default(),
        }))
    }

    fn create_child(&self) -> Self {
        let enclosing = Some(self.clone());
        EnvironmentTrait::new(enclosing.as_ref())
    }

    fn define(&mut self, name: Rc<str>, value: RuntimeValue) {
        self.borrow_mut().values.insert(name, value);
    }

    fn get(&self, name: &Token) -> Option<RuntimeValue> {
        if let Some(val) = self.borrow().values.get(&name.lexeme) {
            Some(val.clone())
        } else if let Some(enclosing) = &self.borrow().enclosing {
            enclosing.get(name)
        } else {
            None
        }
    }

    fn assign(&mut self, name: &Token, value: RuntimeValue) -> RuntimeResult {
        let prev = self.borrow().values.get(&name.lexeme).cloned();
        if let Some(prev) = prev {
            if Environment::check_assignment_type(prev, &value) {
                self.borrow_mut()
                    .values
                    .insert(name.lexeme.clone(), value.clone());
                value.into()
            } else {
                panic!("type error")
            }
        } else if let Some(enclosing) = &mut self.borrow_mut().enclosing {
            enclosing.assign(name, value)
        } else {
            panic!("undefined variable")
        }
    }

    fn get_at(&self, name: &Token, distance: usize) -> RuntimeValue {
        self.ancestor(distance)
            .borrow()
            .values
            .get(&name.lexeme)
            .unwrap_or_else(|| panic!("Variable not found (invalid resolution) {:?}", name))
            .clone()
    }

    fn assign_at(&self, name: &Token, value: RuntimeValue, distance: usize) {
        let ancestor = &self.ancestor(distance);

        ancestor
            .borrow_mut()
            .values
            .insert(name.lexeme.clone(), value);
    }

    fn ancestor(&self, distance: usize) -> Self {
        let mut env = self.clone();
        for _ in 0..distance {
            let enclosing = env.borrow().enclosing.clone().unwrap();
            env = enclosing;
        }
        env
    }
}

impl Environment {
    fn check_assignment_type(prev: RuntimeValue, new: &RuntimeValue) -> bool {
        use RuntimeValue::*;

        matches!(
            (prev, new),
            (String(_), String(_)) | (Integer(_), Integer(_)) | (Bool(_), Bool(_)) | (Unit, _)
        )
    }
}
