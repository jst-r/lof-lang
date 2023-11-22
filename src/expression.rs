use std::{fmt::Debug, rc::Rc};

use crate::{statement::Stmt, token::Token, visitor::AcceptMut};

pub type BoxExpr = Box<Expr>;

#[derive(Debug)]
pub enum Expr {
    Binary {
        left: BoxExpr,
        operator: Token,
        right: BoxExpr,
    },
    Unary {
        operator: Token,
        right: BoxExpr,
    },
    Literal(LiteralExpr),
    Group(BoxExpr),
    Variable(Token),
    Assignment {
        name: Token,
        value: BoxExpr,
    },
    Block(Vec<Stmt>),
    If {
        condition: BoxExpr,
        then_branch: BoxExpr,
        else_branch: Option<BoxExpr>,
    },
    Logical {
        left: BoxExpr,
        operator: Token,
        right: BoxExpr,
    },
    While {
        condition: BoxExpr,
        body: BoxExpr,
    },
    For {
        variable: BoxExpr,
        iterable: BoxExpr,
        body: BoxExpr,
    },
}

#[derive(Debug)]

pub enum LiteralExpr {
    Bool(bool),
    Integer(isize),
    Float(f64),
    String(Rc<str>),
    // Identifier(Token),
}

pub trait ExprVisitor
where
    Self: Sized,
{
    type ReturnType;

    fn visit_binary(
        &mut self,
        left: &BoxExpr,
        operator: &Token,
        right: &BoxExpr,
    ) -> Self::ReturnType;
    fn visit_unary(&mut self, operator: &Token, right: &BoxExpr) -> Self::ReturnType;
    fn visit_literal(&mut self, literal: &LiteralExpr) -> Self::ReturnType;
    fn visit_group(&mut self, expr: &BoxExpr) -> Self::ReturnType;
    fn visit_variable(&mut self, token: &Token) -> Self::ReturnType;
    fn visit_assignment(&mut self, name: &Token, value: &BoxExpr) -> Self::ReturnType;
    fn visit_block(&mut self, stmts: &[Stmt]) -> Self::ReturnType;
    fn visit_if(
        &mut self,
        condition: &BoxExpr,
        then_branch: &BoxExpr,
        else_branch: &Option<BoxExpr>,
    ) -> Self::ReturnType;
    fn visit_logical(
        &mut self,
        left: &BoxExpr,
        operator: &Token,
        right: &BoxExpr,
    ) -> Self::ReturnType;
    fn visit_while(&mut self, condition: &BoxExpr, body: &BoxExpr) -> Self::ReturnType;
    fn visit_for(
        &mut self,
        identifier: &BoxExpr,
        iterable: &BoxExpr,
        body: &BoxExpr,
    ) -> Self::ReturnType;

    fn visit(&mut self, expr: &BoxExpr) -> Self::ReturnType {
        match expr.as_ref() {
            Expr::Binary {
                left,
                operator,
                right,
            } => self.visit_binary(left, operator, right),
            Expr::Unary { operator, right } => self.visit_unary(operator, right),
            Expr::Literal(literal) => self.visit_literal(literal),
            Expr::Group(expr) => self.visit_group(expr),
            Expr::Variable(token) => self.visit_variable(token),
            Expr::Assignment { name, value } => self.visit_assignment(name, value),
            Expr::Block(stmts) => self.visit_block(stmts),
            Expr::If {
                condition,
                then_branch,
                else_branch,
            } => self.visit_if(condition, then_branch, else_branch),
            Expr::Logical {
                left,
                operator,
                right,
            } => self.visit_logical(left, operator, right),
            Expr::While { condition, body } => self.visit_while(condition, body),
            Expr::For {
                variable,
                iterable,
                body,
            } => self.visit_for(variable, iterable, body),
        }
    }
}

impl<V: ExprVisitor> AcceptMut<V, V::ReturnType> for &BoxExpr {
    fn accept(&self, visitor: &mut V) -> V::ReturnType {
        visitor.visit(self)
    }
}
