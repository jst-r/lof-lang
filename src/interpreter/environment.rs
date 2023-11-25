use std::{collections::BTreeMap, rc::Rc};

use super::runtime_type::NativeFunctionWrapper;
use super::runtime_value::RuntimeValue;

use crate::token::Token;

#[derive(Debug)]
pub struct Environment {
    current_ind: usize,
    enclosing_ids: Vec<Option<usize>>,
    value_scopes: Vec<BTreeMap<Rc<str>, RuntimeValue>>,
}

impl Default for Environment {
    fn default() -> Self {
        //TODO move to globals
        fn time(_: [RuntimeValue; 0]) -> RuntimeValue {
            use std::time::{SystemTime, UNIX_EPOCH};
            RuntimeValue::Integer(
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis()
                    .try_into()
                    .unwrap(),
            )
        }
        let time_fn = RuntimeValue::Function(Rc::new(NativeFunctionWrapper { function: time }));
        Self {
            current_ind: 0,
            enclosing_ids: vec![None],
            value_scopes: vec![BTreeMap::from([(Rc::from("now"), time_fn)])],
        }
    }
}

impl Environment {
    pub fn define(&mut self, name: Rc<str>, value: RuntimeValue) {
        self.value_scopes[self.current_ind].insert(name, value);
    }

    pub fn get(&self, name: &Token) -> Option<&RuntimeValue> {
        let mut id = self.current_ind;

        loop {
            if let Some(val) = self.value_scopes[id].get(&name.lexeme) {
                return Some(val);
            } else if let Some(new_id) = self.enclosing_ids[id] {
                id = new_id;
                continue;
            } else {
                return None;
            }
        }
    }

    pub fn assign(&mut self, name: &Token, value: RuntimeValue) -> RuntimeValue {
        use RuntimeValue::*;

        let mut id = self.current_ind;
        let key = &name.lexeme;

        loop {
            if let Some(prev) = self.value_scopes[id].get(&name.lexeme) {
                match (prev, &value) {
                    (String(_), String(_))
                    | (Integer(_), Integer(_))
                    | (Bool(_), Bool(_))
                    | (Unit, Unit) => {
                        self.value_scopes[id].insert(key.clone(), value.clone());
                    }
                    _ => panic!("type error"),
                };

                return value;
            } else if let Some(new_id) = self.enclosing_ids[id] {
                id = new_id;
                continue;
            } else {
                panic!("undefined variable");
            }
        }
    }

    pub fn push(&mut self) {
        self.enclosing_ids.push(Some(self.current_ind));
        self.value_scopes.push(BTreeMap::new());
        self.current_ind = self.value_scopes.len() - 1;
    }

    pub fn pop(&mut self) {
        let next_ind = self.enclosing_ids[self.current_ind];
        self.enclosing_ids.swap_remove(self.current_ind);
        self.value_scopes.swap_remove(self.current_ind);
        self.current_ind = next_ind.expect("No enclosing env");
    }
}
