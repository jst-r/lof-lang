use crate::{expression::BoxExpr, token::Token, visitor::AcceptMut};

#[derive(Debug, Clone)]
pub enum Stmt {
    Print(BoxExpr),
    Expr {
        expr: BoxExpr,
        has_semicolon: bool,
    },
    Var {
        name: Token,
        initializer: Option<BoxExpr>,
    },
    Fn {
        name: Token,
        params: Vec<Token>,
        body: BoxExpr,
    },
}

pub trait StmtVisitor {
    type ReturnType;

    fn visit_print(&mut self, expr: &BoxExpr) -> Self::ReturnType;
    fn visit_expr(&mut self, expr: &BoxExpr) -> Self::ReturnType;
    fn visit_var(&mut self, name: &Token, initializer: &Option<BoxExpr>) -> Self::ReturnType;
    fn visit_function(&mut self, name: &Token, args: &[Token], body: &BoxExpr) -> Self::ReturnType;

    fn visit(&mut self, stmt: &Stmt) -> Self::ReturnType {
        match stmt {
            Stmt::Expr {
                expr,
                has_semicolon: _,
            } => self.visit_expr(expr),
            Stmt::Print(expr) => self.visit_print(expr),
            Stmt::Var { name, initializer } => self.visit_var(name, initializer),
            Stmt::Fn { name, params, body } => self.visit_function(name, params, body),
        }
    }
}

impl<V: StmtVisitor> AcceptMut<V, V::ReturnType> for Stmt {
    fn accept(&self, visitor: &mut V) -> V::ReturnType {
        visitor.visit(self)
    }
}
