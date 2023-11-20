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

    fn visit_binary(&mut self, left: BoxExpr, operator: Token, right: BoxExpr) -> Self::ReturnType;
    fn visit_unary(&mut self, operator: Token, right: BoxExpr) -> Self::ReturnType;
    fn visit_literal(&mut self, literal: LiteralExpr) -> Self::ReturnType;
    fn visit_group(&mut self, expr: BoxExpr) -> Self::ReturnType;
    fn visit_variable(&mut self, token: Token) -> Self::ReturnType;
    fn visit_assignment(&mut self, name: Token, value: BoxExpr) -> Self::ReturnType;
    fn visit_block(&mut self, stmts: Vec<Stmt>) -> Self::ReturnType;

    fn visit(&mut self, expr: BoxExpr) -> Self::ReturnType {
        match *expr {
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
        }
    }
}

impl<V: ExprVisitor> AcceptMut<V, V::ReturnType> for BoxExpr {
    fn accept(self, visitor: &mut V) -> V::ReturnType {
        visitor.visit(self)
    }
}
