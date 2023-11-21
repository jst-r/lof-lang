use crate::{expression::BoxExpr, token::Token, visitor::AcceptMut};

#[derive(Debug)]
pub enum Stmt {
    Print(BoxExpr),
    Expr(BoxExpr),
    Var {
        name: Token,
        initializer: Option<BoxExpr>,
    },
}

pub trait StmtVisitor {
    type ReturnType;

    fn visit_print(&mut self, expr: &BoxExpr) -> Self::ReturnType;
    fn visit_expr(&mut self, expr: &BoxExpr) -> Self::ReturnType;
    fn visit_var(&mut self, name: &Token, initializer: &Option<BoxExpr>) -> Self::ReturnType;

    fn visit(&mut self, stmt: &Stmt) -> Self::ReturnType {
        match stmt {
            Stmt::Expr(expr) => self.visit_expr(expr),
            Stmt::Print(expr) => self.visit_print(expr),
            Stmt::Var { name, initializer } => self.visit_var(name, initializer),
        }
    }
}

impl<V: StmtVisitor> AcceptMut<V, V::ReturnType> for Stmt {
    fn accept(&self, visitor: &mut V) -> V::ReturnType {
        visitor.visit(self)
    }
}
