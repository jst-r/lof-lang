use std::{fmt::Debug, rc::Rc};

use crate::{token::Token, visitor::Accept};

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

    fn visit_binary(&self, left: BoxExpr, operator: Token, right: BoxExpr) -> Self::ReturnType;
    fn visit_unary(&self, operator: Token, right: BoxExpr) -> Self::ReturnType;
    fn visit_literal(&self, literal: LiteralExpr) -> Self::ReturnType;
    fn visit_group(&self, expr: BoxExpr) -> Self::ReturnType;
    fn visit_variable(&self, token: Token) -> Self::ReturnType;

    fn visit(&self, expr: BoxExpr) -> Self::ReturnType {
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
        }
    }
}

impl<V: ExprVisitor> Accept<V, V::ReturnType> for BoxExpr {
    fn accept(self, visitor: &V) -> V::ReturnType {
        visitor.visit(self)
    }
}
