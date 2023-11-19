use std::{collections::BTreeMap, rc::Rc};

use crate::{
    expression::{BoxExpr, ExprVisitor, LiteralExpr},
    statement::StmtVisitor,
    token::{Token, TokenKind},
    visitor::{Accept, AcceptMut},
};

#[derive(Debug, Clone)]
pub enum RuntimeValue {
    String(Rc<str>),
    Integer(isize),
    Float(f64),
    Bool(bool),
    Unit,
}

use RuntimeValue::*;

#[derive(Debug, Default)]
struct Environment {
    values: BTreeMap<Rc<str>, RuntimeValue>,
}

impl Environment {
    fn define(&mut self, name: Rc<str>, value: RuntimeValue) {
        self.values.insert(name, value);
    }

    fn get(&self, name: &Token) -> Option<&RuntimeValue> {
        self.values.get(&name.lexeme)
    }
}

#[derive(Debug, Default)]
pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn interpret(&mut self, program: Vec<crate::statement::Stmt>) {
        for stmt in program {
            stmt.accept(self)
        }
    }

    fn unary_minus(val: RuntimeValue) -> RuntimeValue {
        match val {
            Float(f) => Float(-f),
            Integer(i) => Integer(-i),
            _ => panic!("Invalid type"),
        }
    }

    fn unary_bang(val: RuntimeValue) -> RuntimeValue {
        Bool(match val {
            Integer(i) => i == 0,
            Bool(b) => b,
            String(s) => s.is_empty(),
            _ => panic!("invalid type"),
        })
    }

    fn binary_plus(left: RuntimeValue, right: RuntimeValue) -> RuntimeValue {
        match (left, right) {
            (Float(a), Float(b)) => Float(a + b),
            (Integer(a), Integer(b)) => Integer(a + b),
            (String(a), String(b)) => String(((*a).to_owned() + &*b).into()),
            _ => panic!("invalid type"),
        }
    }

    fn binary_minus(left: RuntimeValue, right: RuntimeValue) -> RuntimeValue {
        match (left, right) {
            (Float(a), Float(b)) => Float(a - b),
            (Integer(a), Integer(b)) => Integer(a - b),
            _ => panic!("invalid type"),
        }
    }

    fn binary_star(left: RuntimeValue, right: RuntimeValue) -> RuntimeValue {
        match (left, right) {
            (Float(a), Float(b)) => Float(a * b),
            (Integer(a), Integer(b)) => Integer(a * b),
            _ => panic!("invalid type"),
        }
    }

    fn binary_slash(left: RuntimeValue, right: RuntimeValue) -> RuntimeValue {
        match (left, right) {
            (Float(a), Float(b)) => Float(a / b),
            (Integer(a), Integer(b)) => Integer(a / b),
            _ => panic!("invalid type"),
        }
    }

    fn binary_greater(left: RuntimeValue, right: RuntimeValue) -> RuntimeValue {
        match (left, right) {
            (Float(a), Float(b)) => Bool(a > b),
            (Integer(a), Integer(b)) => Bool(a > b),
            _ => panic!("invalid type"),
        }
    }

    fn binary_greater_equal(left: RuntimeValue, right: RuntimeValue) -> RuntimeValue {
        match (left, right) {
            (Float(a), Float(b)) => Bool(a >= b),
            (Integer(a), Integer(b)) => Bool(a >= b),
            _ => panic!("invalid type"),
        }
    }

    fn binary_less(left: RuntimeValue, right: RuntimeValue) -> RuntimeValue {
        match (left, right) {
            (Float(a), Float(b)) => Bool(a < b),
            (Integer(a), Integer(b)) => Bool(a < b),
            _ => panic!("invalid type"),
        }
    }

    fn binary_less_equal(left: RuntimeValue, right: RuntimeValue) -> RuntimeValue {
        match (left, right) {
            (Float(a), Float(b)) => Bool(a <= b),
            (Integer(a), Integer(b)) => Bool(a <= b),
            _ => panic!("invalid type"),
        }
    }

    fn binary_bang_equal(left: RuntimeValue, right: RuntimeValue) -> RuntimeValue {
        match (left, right) {
            (Float(a), Float(b)) => Bool(a != b),
            (Integer(a), Integer(b)) => Bool(a != b),
            (String(a), String(b)) => Bool(a != b),
            (Bool(a), Bool(b)) => Bool(a != b),

            _ => panic!("invalid type"),
        }
    }

    fn binary_equal_equal(left: RuntimeValue, right: RuntimeValue) -> RuntimeValue {
        match (left, right) {
            (Float(a), Float(b)) => Bool(a == b),
            (Integer(a), Integer(b)) => Bool(a == b),
            (String(a), String(b)) => Bool(a == b),
            (Bool(a), Bool(b)) => Bool(a == b),
            _ => panic!("invalid type"),
        }
    }
}

impl ExprVisitor for Interpreter {
    type ReturnType = RuntimeValue;

    fn visit_binary(&self, left: BoxExpr, operator: Token, right: BoxExpr) -> Self::ReturnType {
        let left = left.accept(self);
        let right = right.accept(self);

        match operator.kind {
            TokenKind::Plus => Interpreter::binary_plus(left, right),
            TokenKind::Minus => Interpreter::binary_minus(left, right),
            TokenKind::Star => Interpreter::binary_star(left, right),
            TokenKind::Slash => Interpreter::binary_slash(left, right),
            TokenKind::Greater => Interpreter::binary_greater(left, right),
            TokenKind::GreaterEqual => Interpreter::binary_greater_equal(left, right),
            TokenKind::Less => Interpreter::binary_less(left, right),
            TokenKind::LessEqual => Interpreter::binary_less_equal(left, right),
            TokenKind::BangEqual => Interpreter::binary_bang_equal(left, right),
            TokenKind::EqualEqual => Interpreter::binary_equal_equal(left, right),
            _ => panic!("Invalid binary operator"),
        }
    }

    fn visit_unary(&self, operator: Token, right: BoxExpr) -> Self::ReturnType {
        let right = right.accept(self);

        match operator.kind {
            TokenKind::Bang => Interpreter::unary_bang(right),
            TokenKind::Minus => Interpreter::unary_minus(right),
            _ => panic!("Invalid unary operator"),
        }
    }

    fn visit_literal(&self, literal: LiteralExpr) -> Self::ReturnType {
        match literal {
            LiteralExpr::Bool(b) => Bool(b),
            LiteralExpr::Integer(n) => Integer(n),
            LiteralExpr::Float(f) => Float(f),
            LiteralExpr::String(s) => String(s),
        }
    }

    fn visit_group(&self, expr: BoxExpr) -> Self::ReturnType {
        expr.accept(self)
    }

    fn visit_variable(&self, token: Token) -> Self::ReturnType {
        dbg!(&self);
        dbg!(&token);
        match self.environment.get(&token) {
            Some(t) => t.clone(),
            Option::None => panic!("undefined variable"),
        }
    }
}

impl StmtVisitor for Interpreter {
    type ReturnType = ();

    fn visit_print(&self, expr: BoxExpr) -> Self::ReturnType {
        println!("{:?}", expr.accept(self))
    }

    fn visit_expr(&self, expr: BoxExpr) -> Self::ReturnType {
        expr.accept(self);
    }

    fn visit_var(&mut self, name: Token, initializer: Option<BoxExpr>) -> Self::ReturnType {
        dbg!(&name);
        self.environment.define(
            name.lexeme.clone(),
            match initializer {
                Some(init) => init.accept(self),
                Option::None => Unit,
            },
        );
        dbg!(&self);
    }
}
