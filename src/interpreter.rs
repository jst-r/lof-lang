use std::{collections::BTreeMap, fmt::Debug, rc::Rc};

use crate::{
    expression::{BoxExpr, Expr, ExprVisitor, LiteralExpr},
    statement::{Stmt, StmtVisitor},
    token::{Token, TokenKind},
    visitor::AcceptMut,
};

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

use RuntimeValue::*;

#[derive(Debug)]
struct Environment {
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
        let time_fn = Function(Rc::new(runtime_type::NativeFunctionWrapper {
            function: time,
        }));
        Self {
            current_ind: 0,
            enclosing_ids: vec![None],
            value_scopes: vec![BTreeMap::from([(Rc::from("now"), time_fn)])],
        }
    }
}

impl Environment {
    fn define(&mut self, name: Rc<str>, value: RuntimeValue) {
        self.value_scopes[self.current_ind].insert(name, value);
    }

    fn get(&self, name: &Token) -> Option<&RuntimeValue> {
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

    fn assign(&mut self, name: &Token, value: RuntimeValue) -> RuntimeValue {
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

    fn push(&mut self) {
        self.enclosing_ids.push(Some(self.current_ind));
        self.value_scopes.push(BTreeMap::new());
        self.current_ind = self.value_scopes.len() - 1;
    }

    fn pop(&mut self) {
        let next_ind = self.enclosing_ids[self.current_ind];
        self.enclosing_ids.swap_remove(self.current_ind);
        self.value_scopes.swap_remove(self.current_ind);
        self.current_ind = next_ind.expect("No enclosing env");
    }
}

#[derive(Debug, Default)]
pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn interpret(&mut self, program: Vec<Stmt>) {
        for stmt in program {
            stmt.accept(self)
        }
    }

    fn is_truthy(val: &RuntimeValue) -> bool {
        match val {
            Integer(i) => *i != 0,
            Bool(b) => *b,
            String(s) => !s.is_empty(),
            _ => panic!("invalid type"),
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
        Bool(!Interpreter::is_truthy(&val))
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

    fn binary_dot_dot(left: RuntimeValue, right: RuntimeValue) -> RuntimeValue {
        match (left, right) {
            (Integer(from), Integer(to)) => Range(from, to),
            _ => panic!("invalid type"),
        }
    }

    fn call(&mut self, callee: &BoxExpr, args: Vec<RuntimeValue>) -> RuntimeValue {
        let RuntimeValue::Function(callee) = callee.accept(self) else {
            panic!("trying to call not callable")
        };

        if callee.arity() != args.len() {
            panic!("invalid number of arguments")
        }

        callee.call(self, args)
    }
}

impl ExprVisitor for Interpreter {
    type ReturnType = RuntimeValue;

    fn visit_binary(
        &mut self,
        left: &BoxExpr,
        operator: &Token,
        right: &BoxExpr,
    ) -> Self::ReturnType {
        let left = left.accept(self);
        let right = right.accept(self);

        use TokenKind::*;

        match operator.kind {
            Plus => Interpreter::binary_plus(left, right),
            Minus => Interpreter::binary_minus(left, right),
            Star => Interpreter::binary_star(left, right),
            Slash => Interpreter::binary_slash(left, right),
            Greater => Interpreter::binary_greater(left, right),
            GreaterEqual => Interpreter::binary_greater_equal(left, right),
            Less => Interpreter::binary_less(left, right),
            LessEqual => Interpreter::binary_less_equal(left, right),
            BangEqual => Interpreter::binary_bang_equal(left, right),
            EqualEqual => Interpreter::binary_equal_equal(left, right),
            DotDot => Interpreter::binary_dot_dot(left, right),
            _ => panic!("Invalid binary operator"),
        }
    }

    fn visit_unary(&mut self, operator: &Token, right: &BoxExpr) -> Self::ReturnType {
        let right = right.accept(self);

        match operator.kind {
            TokenKind::Bang => Interpreter::unary_bang(right),
            TokenKind::Minus => Interpreter::unary_minus(right),
            _ => panic!("Invalid unary operator"),
        }
    }

    fn visit_literal(&mut self, literal: &LiteralExpr) -> Self::ReturnType {
        match literal {
            LiteralExpr::Bool(b) => Bool(*b),
            LiteralExpr::Integer(n) => Integer(*n),
            LiteralExpr::Float(f) => Float(*f),
            LiteralExpr::String(s) => String(s.clone()),
        }
    }

    fn visit_group(&mut self, expr: &BoxExpr) -> Self::ReturnType {
        expr.accept(self)
    }

    fn visit_variable(&mut self, token: &Token) -> Self::ReturnType {
        match self.environment.get(token) {
            Some(t) => t.clone(),
            Option::None => panic!("undefined variable"),
        }
    }

    fn visit_assignment(&mut self, name: &Token, value: &BoxExpr) -> Self::ReturnType {
        let value = value.accept(self);

        self.environment.assign(name, value)
    }

    fn visit_block(
        &mut self,
        stmts: &[crate::statement::Stmt],
        return_expr: &Option<Box<Expr>>,
    ) -> Self::ReturnType {
        self.environment.push();

        for stmt in stmts {
            stmt.accept(self);
        }

        let return_value = if let Some(return_expr) = return_expr {
            return_expr.accept(self)
        } else {
            RuntimeValue::Unit
        };

        self.environment.pop();

        return_value
    }

    fn visit_if(
        &mut self,
        condition: &BoxExpr,
        then_branch: &BoxExpr,
        else_branch: &Option<BoxExpr>,
    ) -> Self::ReturnType {
        if Interpreter::is_truthy(&condition.accept(self)) {
            then_branch.accept(self)
        } else if let Some(else_branch) = else_branch {
            else_branch.accept(self)
        } else {
            Unit
        }
    }

    fn visit_logical(
        &mut self,
        left: &BoxExpr,
        operator: &Token,
        right: &BoxExpr,
    ) -> Self::ReturnType {
        let left_val = left.accept(self);
        let left_truthy = Interpreter::is_truthy(&left_val);

        if operator.kind == TokenKind::And {
            if !left_truthy {
                return left_val;
            };
        } else if operator.kind == TokenKind::Or {
            if left_truthy {
                return left_val;
            };
        } else {
            panic!("Invalid logical operator");
        };

        right.accept(self)
    }

    fn visit_while(&mut self, condition: &BoxExpr, body: &BoxExpr) -> Self::ReturnType {
        while Interpreter::is_truthy(&condition.accept(self)) {
            body.accept(self);
        }

        Unit
    }

    fn visit_for(
        &mut self,
        variable: &BoxExpr,
        iterable: &BoxExpr,
        body: &BoxExpr,
    ) -> Self::ReturnType {
        let identifier = match variable.as_ref() {
            Expr::Variable(tok) => tok,
            _ => panic!("expected a variable"),
        };
        if let Range(low, high) = iterable.accept(self) {
            for i in low..high {
                self.environment.push();
                let lexeme = identifier.lexeme.clone();
                self.environment.define(lexeme, Integer(i));
                body.accept(self);
                self.environment.pop();
            }
            Unit
        } else {
            panic!("invalid type")
        }
    }

    fn visit_call(&mut self, callee: &BoxExpr, _: &Token, args: &[BoxExpr]) -> Self::ReturnType {
        let args = args.iter().map(|arg| arg.accept(self)).collect::<Vec<_>>();

        self.call(&callee, args)
    }
}

impl StmtVisitor for Interpreter {
    type ReturnType = ();

    fn visit_print(&mut self, expr: &BoxExpr) -> Self::ReturnType {
        println!("{:?}", expr.accept(self))
    }

    fn visit_expr(&mut self, expr: &BoxExpr) -> Self::ReturnType {
        expr.accept(self);
    }

    fn visit_var(&mut self, name: &Token, initializer: &Option<BoxExpr>) -> Self::ReturnType {
        let value = match initializer {
            Some(init) => init.accept(self),
            Option::None => Unit,
        };
        self.environment.define(name.lexeme.clone(), value);
    }

    fn visit_function(
        &mut self,
        name: &Token,
        args: &[Token],
        body: &Box<Expr>,
    ) -> Self::ReturnType {
        let runtime_decl = Function(Rc::new(runtime_type::Function {
            name: name.clone(),
            args: args.clone().into(),
            body: body.clone(),
        }));

        self.environment.define(name.lexeme.clone(), runtime_decl);
    }
}

#[allow(dead_code)]
pub mod runtime_type {
    use std::{fmt::Debug, iter::zip};

    use crate::{expression::BoxExpr, token::Token, visitor::AcceptMut};

    use super::{Interpreter, RuntimeValue};

    // Considering making all runtime values a struct with a marker trait. But that seems like a pain

    pub trait Callable: Debug {
        fn call(&self, interpreter: &mut Interpreter, args: Vec<RuntimeValue>) -> RuntimeValue;
        fn arity(&self) -> usize;
    }

    #[derive(Debug, Clone)]
    pub struct Function {
        pub name: Token,
        pub args: Vec<Token>,
        pub body: BoxExpr,
    }

    impl Callable for Function {
        fn call(&self, interpreter: &mut Interpreter, args: Vec<RuntimeValue>) -> RuntimeValue {
            interpreter.environment.push();

            for (arg_token, arg_val) in zip(self.args.iter(), args) {
                interpreter
                    .environment
                    .define(arg_token.lexeme.clone(), arg_val);
            }

            let return_value = (&self.body).accept(interpreter);

            interpreter.environment.pop();

            return_value
        }

        fn arity(&self) -> usize {
            self.args.len()
        }
    }

    // kinda proud of this one
    pub struct NativeFunctionWrapper<const N: usize, F: Fn([RuntimeValue; N]) -> RuntimeValue> {
        pub function: F,
    }

    impl<const N: usize, F: Fn([RuntimeValue; N]) -> RuntimeValue> Callable
        for NativeFunctionWrapper<N, F>
    {
        fn call(&self, _: &mut Interpreter, args: Vec<RuntimeValue>) -> RuntimeValue {
            (self.function)(args.try_into().expect("invalid number of arguments"))
        }

        fn arity(&self) -> usize {
            N
        }
    }

    impl<const N: usize, F: Fn([RuntimeValue; N]) -> RuntimeValue> Debug
        for NativeFunctionWrapper<N, F>
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("NativeFunctionWrapper")
                .field("function", &"<native function>")
                .field("arity", &N)
                .finish()
        }
    }
}
