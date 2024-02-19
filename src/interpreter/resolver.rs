use std::{collections::BTreeMap, rc::Rc};

use crate::{
    expression::{BoxExpr, ExprVisitor, LiteralExpr},
    statement::{Stmt, StmtVisitor},
    token::Token,
    visitor::AcceptMut,
};

#[derive(Default)]
pub struct Resolver {
    scopes: Vec<BTreeMap<Rc<str>, bool>>,
    pub resolutions: BTreeMap<usize, usize>,
}
impl Resolver {
    pub fn resolver_pass(&mut self, stmts: &[Stmt]) {
        for stmt in stmts {
            stmt.accept(self);
        }
    }

    fn begin_scope(&mut self) {
        self.scopes.push(Default::default());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn add_resolution(&mut self, token: &Token, depth: usize) {
        self.resolutions.insert(token.id, depth);
    }

    fn declare(&mut self, name: &Token) {
        if self.scopes.is_empty() {
            return;
        }

        self.scopes
            .last_mut()
            .unwrap()
            .insert(name.lexeme.clone(), false);
    }

    fn define(&mut self, name: &Token) {
        if self.scopes.is_empty() {
            return;
        }

        self.scopes
            .last_mut()
            .unwrap()
            .insert(name.lexeme.clone(), true);
    }

    fn resolve_local(&mut self, token: &Token) {
        for i in (0..self.scopes.len()).rev() {
            if self.scopes[i].get(&token.lexeme).is_some() {
                self.add_resolution(token, self.scopes.len() - i - 1);
                return;
            }
        }
    }

    fn resolve_function(&mut self, args: &[Token], body: &BoxExpr) {
        self.begin_scope();
        for arg in args {
            self.declare(arg);
            self.define(arg);
        }

        body.accept(self);
        self.end_scope();
    }
}

impl ExprVisitor for Resolver {
    type ReturnType = ();

    fn visit_binary(&mut self, left: &BoxExpr, _: &Token, right: &BoxExpr) -> Self::ReturnType {
        left.accept(self);
        right.accept(self);
    }

    fn visit_unary(&mut self, _: &Token, right: &BoxExpr) -> Self::ReturnType {
        right.accept(self);
    }

    fn visit_literal(&mut self, _: &LiteralExpr) -> Self::ReturnType {}

    fn visit_group(&mut self, expr: &BoxExpr) -> Self::ReturnType {
        expr.accept(self);
    }

    fn visit_variable(&mut self, token: &Token) -> Self::ReturnType {
        if !self.scopes.is_empty() && self.scopes.last().unwrap().get(&token.lexeme) == Some(&false)
        {
            panic!("can not use variable in its own declaration")
        }
        self.resolve_local(token);
    }

    fn visit_assignment(&mut self, name: &Token, value: &BoxExpr) -> Self::ReturnType {
        value.accept(self);
        self.resolve_local(name);
    }

    fn visit_block(&mut self, stmts: &[Stmt], return_expr: &Option<BoxExpr>) -> Self::ReturnType {
        self.begin_scope();

        for stmt in stmts {
            stmt.accept(self);
        }

        if let Some(e) = return_expr.as_ref() {
            (e).accept(self)
        }

        self.end_scope();
    }

    fn visit_if(
        &mut self,
        condition: &BoxExpr,
        then_branch: &BoxExpr,
        else_branch: &Option<BoxExpr>,
    ) -> Self::ReturnType {
        condition.accept(self);
        then_branch.accept(self);
        if let Some(e) = else_branch.as_ref() {
            e.accept(self)
        }
    }

    fn visit_logical(&mut self, left: &BoxExpr, _: &Token, right: &BoxExpr) -> Self::ReturnType {
        left.accept(self);
        right.accept(self);
    }

    fn visit_while(&mut self, condition: &BoxExpr, body: &BoxExpr) -> Self::ReturnType {
        condition.accept(self);
        body.accept(self);
    }

    fn visit_for(
        &mut self,
        identifier: &BoxExpr,
        iterable: &BoxExpr,
        body: &BoxExpr,
    ) -> Self::ReturnType {
        identifier.accept(self);
        iterable.accept(self);
        body.accept(self);
    }

    fn visit_call(&mut self, callee: &BoxExpr, _: &Token, args: &[BoxExpr]) -> Self::ReturnType {
        callee.accept(self);
        for arg in args {
            arg.accept(self);
        }
    }

    fn visit_return(&mut self, _: &Token, value: &Option<BoxExpr>) -> Self::ReturnType {
        if let Some(e) = value.as_ref() {
            e.accept(self)
        }
    }

    fn visit_field_access(&mut self, object: &BoxExpr, _name: &Token) -> Self::ReturnType {
        object.accept(self);
    }

    fn visit_filed_set(
        &mut self,
        object: &BoxExpr,
        _name: &Token,
        value: &BoxExpr,
    ) -> Self::ReturnType {
        value.accept(self);
        object.accept(self);
    }
}

impl StmtVisitor for Resolver {
    type ReturnType = ();

    fn visit_print(&mut self, expr: &BoxExpr) -> Self::ReturnType {
        expr.accept(self);
    }

    fn visit_expr(&mut self, expr: &BoxExpr) -> Self::ReturnType {
        expr.accept(self);
    }

    fn visit_var(&mut self, name: &Token, initializer: &Option<BoxExpr>) -> Self::ReturnType {
        self.declare(name);
        if let Some(initializer) = initializer {
            initializer.accept(self);
        }
        self.define(name);
    }

    fn visit_function(&mut self, name: &Token, args: &[Token], body: &BoxExpr) -> Self::ReturnType {
        self.declare(name);
        self.define(name);

        self.resolve_function(args, body);
    }

    fn visit_class(&mut self, name: &Token, _methods: &[Stmt]) -> Self::ReturnType {
        self.declare(name);
        self.define(name);
    }
}
