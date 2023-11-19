use std::fmt::Debug;

use thiserror::Error;

use crate::{
    expression::{BoxExpr, Expr, LiteralExpr},
    statement::Stmt,
    token::{LiteralValue, Token, TokenKind},
};

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("unexpected token: {0:?}")]
    UnexpectedToken(Token),
    #[error("parenthesis not closed after {0:?}")]
    ParenthesisNotClosed(Token),
    #[error("missing semicolon, expected after {0:?}")]
    MissingSemicolon(Token),
    #[error("expected variable name in place of {0:?}")]
    InvalidVariableName(Token),
    #[error("invalid literal {0:?}")]
    InvalidLiteral(Token),
}

pub type ExprResult = Result<BoxExpr, ParserError>;
pub type StmtResult = Result<Stmt, ParserError>;

fn wrap_expr(expr: Expr) -> ExprResult {
    Ok(Box::new(expr))
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Vec<StmtResult> {
        let mut statements = vec![];
        while !self.is_at_end() {
            statements.push(self.declaration());
            dbg!(&statements);
            dbg!(self.peek());
        }

        statements

        // self.expression()
    }

    fn declaration(&mut self) -> StmtResult {
        if self.matches(&[TokenKind::Var]) {
            Ok(self.var_declaration()?)
        } else {
            Ok(self.statement()?)
        }
    }

    fn var_declaration(&mut self) -> StmtResult {
        let name = self
            .consume(
                TokenKind::Identifier,
                ParserError::InvalidVariableName(self.peek().clone()),
            )?
            .clone();

        let initializer = if self.matches(&[TokenKind::Equal]) {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(TokenKind::Semicolon, self.err_missing_semi())?;

        Ok(Stmt::Var { initializer, name })
    }

    fn statement(&mut self) -> StmtResult {
        if self.matches(&[TokenKind::Print]) {
            self.print_statement()
        } else {
            self.expression_statement()
        }
    }

    fn print_statement(&mut self) -> StmtResult {
        let expr = self.expression()?;
        self.consume(TokenKind::Semicolon, self.err_missing_semi())?;

        Ok(Stmt::Print(expr))
    }

    fn expression_statement(&mut self) -> StmtResult {
        let expr = self.expression()?;
        self.consume(TokenKind::Semicolon, self.err_missing_semi())?;

        Ok(Stmt::Expr(expr))
    }

    fn expression(&mut self) -> ExprResult {
        self.equality()
    }

    fn equality(&mut self) -> ExprResult {
        let mut expr = self.comparison()?;

        while self.matches(&[TokenKind::BangEqual, TokenKind::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;

            expr = Box::new(Expr::Binary {
                left: expr,
                operator,
                right,
            });
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> ExprResult {
        let mut expr = self.term()?;

        while self.matches(&[
            TokenKind::Less,
            TokenKind::LessEqual,
            TokenKind::Greater,
            TokenKind::GreaterEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Box::new(Expr::Binary {
                left: expr,
                operator,
                right,
            })
        }

        Ok(expr)
    }

    fn term(&mut self) -> ExprResult {
        let mut expr = self.factor()?;

        while self.matches(&[TokenKind::Plus, TokenKind::Minus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Box::new(Expr::Binary {
                left: expr,
                operator,
                right,
            })
        }

        Ok(expr)
    }

    fn factor(&mut self) -> ExprResult {
        let mut expr = self.unary()?;

        while self.matches(&[TokenKind::Star, TokenKind::Slash]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Box::new(Expr::Binary {
                left: expr,
                operator,
                right,
            })
        }

        Ok(expr)
    }

    fn unary(&mut self) -> ExprResult {
        if self.matches(&[TokenKind::Bang, TokenKind::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            wrap_expr(Expr::Unary { operator, right })
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> ExprResult {
        if self.matches(&[TokenKind::Identifier]) {
            let prev = self.previous();
            return wrap_expr(Expr::Variable(prev.clone()));
        }

        if self.matches(&[TokenKind::Literal]) {
            let prev = self.previous();
            let literal_expr = match prev.literal.clone() {
                LiteralValue::String(s) => Ok(Expr::Literal(LiteralExpr::String(s.into()))),
                LiteralValue::Float(f) => Ok(Expr::Literal(LiteralExpr::Float(f))),
                LiteralValue::Integer(n) => Ok(Expr::Literal(LiteralExpr::Integer(n))),
                _ => Err(ParserError::InvalidLiteral(prev.clone())),
            }?;
            return wrap_expr(literal_expr);
        }

        if self.matches(&[TokenKind::True, TokenKind::False]) {
            let literal_expr = match self.previous().kind {
                TokenKind::True => Ok(Expr::Literal(LiteralExpr::Bool(true))),
                TokenKind::False => Ok(Expr::Literal(LiteralExpr::Bool(false))),
                _ => Err(ParserError::InvalidLiteral(self.previous().clone())),
            }?;
            return wrap_expr(literal_expr);
        }

        if self.matches(&[TokenKind::LeftParen]) {
            let expr = self.expression()?;
            if self.advance().kind != TokenKind::RightParen {
                return Err(ParserError::ParenthesisNotClosed(self.previous().clone()));
            }
            return wrap_expr(Expr::Group(expr));
        }

        Err(ParserError::UnexpectedToken(self.peek().clone()))
    }

    fn matches(&mut self, kinds: &[TokenKind]) -> bool {
        for kind in kinds {
            if self.check(*kind) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, kind: TokenKind) -> bool {
        !self.is_at_end() && self.peek().kind == kind
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn consume(&mut self, token: TokenKind, error: ParserError) -> Result<&Token, ParserError> {
        let tok = self.advance();
        if self.check(token) {
            Ok(tok)
        } else {
            Err(error)
        }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn is_at_end(&self) -> bool {
        self.peek().kind == TokenKind::Eof
    }

    fn err_missing_semi(&self) -> ParserError {
        ParserError::MissingSemicolon(self.previous().clone())
    }
}
