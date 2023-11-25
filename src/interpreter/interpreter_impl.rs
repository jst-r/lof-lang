use std::rc::Rc;

use super::{
    runtime_type,
    runtime_value::{RuntimeResult, RuntimeResultNoValue, RuntimeValue},
    Interpreter,
};

use crate::{
    expression::{BoxExpr, Expr, ExprVisitor, LiteralExpr},
    statement::{Stmt, StmtVisitor},
    token::{Token, TokenKind},
    visitor::AcceptMut,
};

use RuntimeValue::*;
impl Interpreter {
    pub fn interpret(&mut self, program: Vec<Stmt>) -> RuntimeResultNoValue {
        for stmt in program {
            stmt.accept(self)?;
        }

        Ok(())
    }

    fn is_truthy(val: &RuntimeValue) -> bool {
        match val {
            Integer(i) => *i != 0,
            Bool(b) => *b,
            String(s) => !s.is_empty(),
            _ => panic!("invalid type"),
        }
    }

    fn unary_minus(val: RuntimeValue) -> RuntimeResult {
        match val {
            Float(f) => Float(-f),
            Integer(i) => Integer(-i),
            _ => panic!("Invalid type"),
        }
        .into()
    }

    fn unary_bang(val: RuntimeValue) -> RuntimeResult {
        Bool(!Interpreter::is_truthy(&val)).into()
    }

    fn binary_plus(left: RuntimeValue, right: RuntimeValue) -> RuntimeResult {
        match (left, right) {
            (Float(a), Float(b)) => Float(a + b),
            (Integer(a), Integer(b)) => Integer(a + b),
            (String(a), String(b)) => String(((*a).to_owned() + &*b).into()),
            _ => panic!("invalid type"),
        }
        .into()
    }

    fn binary_minus(left: RuntimeValue, right: RuntimeValue) -> RuntimeResult {
        match (left, right) {
            (Float(a), Float(b)) => Float(a - b),
            (Integer(a), Integer(b)) => Integer(a - b),
            _ => panic!("invalid type"),
        }
        .into()
    }

    fn binary_star(left: RuntimeValue, right: RuntimeValue) -> RuntimeResult {
        match (left, right) {
            (Float(a), Float(b)) => Float(a * b),
            (Integer(a), Integer(b)) => Integer(a * b),
            _ => panic!("invalid type"),
        }
        .into()
    }

    fn binary_slash(left: RuntimeValue, right: RuntimeValue) -> RuntimeResult {
        match (left, right) {
            (Float(a), Float(b)) => Float(a / b),
            (Integer(a), Integer(b)) => Integer(a / b),
            _ => panic!("invalid type"),
        }
        .into()
    }

    fn binary_greater(left: RuntimeValue, right: RuntimeValue) -> RuntimeResult {
        match (left, right) {
            (Float(a), Float(b)) => Bool(a > b),
            (Integer(a), Integer(b)) => Bool(a > b),
            _ => panic!("invalid type"),
        }
        .into()
    }

    fn binary_greater_equal(left: RuntimeValue, right: RuntimeValue) -> RuntimeResult {
        match (left, right) {
            (Float(a), Float(b)) => Bool(a >= b),
            (Integer(a), Integer(b)) => Bool(a >= b),
            _ => panic!("invalid type"),
        }
        .into()
    }

    fn binary_less(left: RuntimeValue, right: RuntimeValue) -> RuntimeResult {
        match (left, right) {
            (Float(a), Float(b)) => Bool(a < b),
            (Integer(a), Integer(b)) => Bool(a < b),
            _ => panic!("invalid type"),
        }
        .into()
    }

    fn binary_less_equal(left: RuntimeValue, right: RuntimeValue) -> RuntimeResult {
        match (left, right) {
            (Float(a), Float(b)) => Bool(a <= b),
            (Integer(a), Integer(b)) => Bool(a <= b),
            _ => panic!("invalid type"),
        }
        .into()
    }

    fn binary_bang_equal(left: RuntimeValue, right: RuntimeValue) -> RuntimeResult {
        match (left, right) {
            (Float(a), Float(b)) => Bool(a != b),
            (Integer(a), Integer(b)) => Bool(a != b),
            (String(a), String(b)) => Bool(a != b),
            (Bool(a), Bool(b)) => Bool(a != b),

            _ => panic!("invalid type"),
        }
        .into()
    }

    fn binary_equal_equal(left: RuntimeValue, right: RuntimeValue) -> RuntimeResult {
        match (left, right) {
            (Float(a), Float(b)) => Bool(a == b),
            (Integer(a), Integer(b)) => Bool(a == b),
            (String(a), String(b)) => Bool(a == b),
            (Bool(a), Bool(b)) => Bool(a == b),
            _ => panic!("invalid type"),
        }
        .into()
    }

    fn binary_dot_dot(left: RuntimeValue, right: RuntimeValue) -> RuntimeResult {
        match (left, right) {
            (Integer(from), Integer(to)) => Range(from, to),
            _ => panic!("invalid type"),
        }
        .into()
    }

    fn call(&mut self, callee: &BoxExpr, args: Vec<RuntimeValue>) -> RuntimeResult {
        let RuntimeValue::Function(callee) = callee.accept(self)? else {
            panic!("trying to call not callable")
        };

        if callee.arity() != args.len() {
            panic!("invalid number of arguments")
        }

        callee.call(self, args)
    }
}

impl ExprVisitor for Interpreter {
    type ReturnType = RuntimeResult;

    fn visit_binary(
        &mut self,
        left: &BoxExpr,
        operator: &Token,
        right: &BoxExpr,
    ) -> Self::ReturnType {
        let left = left.accept(self)?;
        let right = right.accept(self)?;

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
        .into()
    }

    fn visit_unary(&mut self, operator: &Token, right: &BoxExpr) -> Self::ReturnType {
        let right = right.accept(self)?;

        match operator.kind {
            TokenKind::Bang => Interpreter::unary_bang(right),
            TokenKind::Minus => Interpreter::unary_minus(right),
            _ => panic!("Invalid unary operator"),
        }
        .into()
    }

    fn visit_literal(&mut self, literal: &LiteralExpr) -> Self::ReturnType {
        match literal {
            LiteralExpr::Bool(b) => Bool(*b),
            LiteralExpr::Integer(n) => Integer(*n),
            LiteralExpr::Float(f) => Float(*f),
            LiteralExpr::String(s) => String(s.clone()),
        }
        .into()
    }

    fn visit_group(&mut self, expr: &BoxExpr) -> Self::ReturnType {
        expr.accept(self)
    }

    fn visit_variable(&mut self, token: &Token) -> Self::ReturnType {
        match self.environment.get(token) {
            Some(t) => t.clone(),
            Option::None => panic!("undefined variable"),
        }
        .into()
    }

    fn visit_assignment(&mut self, name: &Token, value: &BoxExpr) -> Self::ReturnType {
        let value = value.accept(self)?;

        self.environment.assign(name, value)
    }

    fn visit_block(
        &mut self,
        stmts: &[crate::statement::Stmt],
        return_expr: &Option<Box<Expr>>,
    ) -> Self::ReturnType {
        self.environment.push();

        for stmt in stmts {
            stmt.accept(self)?;
        }

        let return_value = if let Some(return_expr) = return_expr {
            return_expr.accept(self)
        } else {
            RuntimeValue::Unit.into()
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
        if Interpreter::is_truthy(&condition.accept(self)?) {
            then_branch.accept(self)
        } else if let Some(else_branch) = else_branch {
            else_branch.accept(self)
        } else {
            Unit.into()
        }
    }

    fn visit_logical(
        &mut self,
        left: &BoxExpr,
        operator: &Token,
        right: &BoxExpr,
    ) -> Self::ReturnType {
        let left_val = left.accept(self)?;
        let left_truthy = Interpreter::is_truthy(&left_val);

        if operator.kind == TokenKind::And {
            if !left_truthy {
                return left_val.into();
            };
        } else if operator.kind == TokenKind::Or {
            if left_truthy {
                return left_val.into();
            };
        } else {
            panic!("Invalid logical operator");
        };

        right.accept(self)
    }

    fn visit_while(&mut self, condition: &BoxExpr, body: &BoxExpr) -> Self::ReturnType {
        while Interpreter::is_truthy(&condition.accept(self)?) {
            body.accept(self)?;
        }

        Unit.into()
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
        if let Range(low, high) = iterable.accept(self)? {
            for i in low..high {
                self.environment.push();
                let lexeme = identifier.lexeme.clone();
                self.environment.define(lexeme, Integer(i));
                body.accept(self)?;
                self.environment.pop();
            }
            Unit.into()
        } else {
            panic!("invalid type")
        }
    }

    fn visit_call(&mut self, callee: &BoxExpr, _: &Token, args: &[BoxExpr]) -> Self::ReturnType {
        let args = args
            .iter()
            .map(|arg| arg.accept(self))
            .collect::<Result<_, _>>()?;

        self.call(callee, args)
    }
}

impl StmtVisitor for Interpreter {
    type ReturnType = RuntimeResultNoValue;

    fn visit_print(&mut self, expr: &BoxExpr) -> Self::ReturnType {
        println!("{:?}", expr.accept(self));
        Ok(())
    }

    fn visit_expr(&mut self, expr: &BoxExpr) -> Self::ReturnType {
        expr.accept(self)?;
        Ok(())
    }

    fn visit_var(&mut self, name: &Token, initializer: &Option<BoxExpr>) -> Self::ReturnType {
        let value = match initializer {
            Some(init) => init.accept(self),
            Option::None => Unit.into(),
        }?;
        self.environment.define(name.lexeme.clone(), value);
        Ok(())
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

        Ok(())
    }
}
