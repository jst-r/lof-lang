use std::fmt::Debug;

use thiserror::Error;

use crate::{
    expression::{BoxExpr, Expr, LiteralExpr},
    statement::Stmt,
    token::{LiteralValue, Token, TokenKind},
};

#[derive(Error, Debug, PartialEq)]
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
    #[error("invalid assignment target {0:?}")]
    InvalidAssignmentTarget(Token),
    #[error("block not closed {0:?}")]
    UnclosedBlock(Token),
    #[error("expected expression, found {0:?}")]
    ExpectedExpression(Token),
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
            let stmt = self.declaration();
            if stmt.is_err() {
                self.synchronize();
            }
            statements.push(stmt);
        }

        statements

        // self.expression()
    }

    fn declaration(&mut self) -> StmtResult {
        if self.matches([TokenKind::Var]) {
            self.var_declaration()
        } else if self.matches([TokenKind::Fn]) {
            self.fn_declaration()
        } else if self.matches([TokenKind::Class]) {
            self.class_declaration()
        } else {
            self.statement()
        }
    }

    fn var_declaration(&mut self) -> StmtResult {
        let name = self
            .consume(
                TokenKind::Identifier,
                ParserError::InvalidVariableName(self.peek().clone()),
            )?
            .clone();

        let initializer = if self.matches([TokenKind::Equal]) {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(TokenKind::Semicolon, self.err_missing_semi())?;

        Ok(Stmt::Var { initializer, name })
    }

    fn fn_declaration(&mut self) -> StmtResult {
        let name = self
            .consume(
                TokenKind::Identifier,
                ParserError::InvalidVariableName(self.peek().clone()), // Expected a function name
            )?
            .clone();

        self.consume(
            TokenKind::LeftParen,
            ParserError::UnexpectedToken(self.peek().clone()),
        )?;

        let mut params = vec![];
        if !self.check(TokenKind::RightParen) {
            loop {
                if params.len() >= 255 {
                    return Err(ParserError::UnexpectedToken(self.peek().clone()));
                    // Max n params exceeded
                }

                params.push(
                    self.consume(
                        TokenKind::Identifier,
                        ParserError::InvalidVariableName(self.peek().clone()), // Expected a parameter name
                    )?
                    .clone(),
                );

                if !self.matches([TokenKind::Comma]) {
                    break;
                }
            }
        }

        self.consume(
            TokenKind::RightParen,
            ParserError::UnexpectedToken(self.peek().clone()),
        )?;

        self.consume(
            TokenKind::LeftCurly,
            ParserError::UnexpectedToken(self.peek().clone()),
        )?;

        let body = self.block()?;

        match body.as_ref() {
            Expr::Block {
                stmts: _,
                return_expr: _,
            } => {}
            _ => panic!("function body isn't a block"),
        };

        Ok(Stmt::Fn { name, params, body })
    }

    fn class_declaration(&mut self) -> Result<Stmt, ParserError> {
        let name = self
            .consume(
                TokenKind::Identifier,
                ParserError::UnexpectedToken(self.peek().clone()),
            )?
            .clone();

        self.consume(
            TokenKind::LeftCurly,
            ParserError::UnexpectedToken(self.peek().clone()),
        )?;

        let mut methods = vec![];

        while !(self.check(TokenKind::RightCurly)) && !self.is_at_end() {
            methods.push(self.fn_declaration()?);
        }

        self.consume(
            TokenKind::RightCurly,
            ParserError::UnexpectedToken(self.peek().clone()),
        )?;

        Ok(Stmt::ClassDeclaration { name, methods })
    }

    fn statement(&mut self) -> StmtResult {
        if self.matches([TokenKind::Print]) {
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
        let has_semicolon = self.matches([TokenKind::Semicolon]);

        Ok(Stmt::Expr {
            expr,
            has_semicolon,
        })
    }

    pub fn expression(&mut self) -> ExprResult {
        match self.peek().kind {
            TokenKind::LeftCurly | TokenKind::If | TokenKind::While | TokenKind::For => {
                self.expression_with_block()
            }
            _ => self.return_expr(),
        }
    }

    fn expression_with_block(&mut self) -> ExprResult {
        if self.matches([TokenKind::LeftCurly]) {
            self.block()
        } else if self.matches([TokenKind::If]) {
            self.if_expr()
        } else if self.matches([TokenKind::While]) {
            self.while_loop()
        } else if self.matches([TokenKind::For]) {
            self.for_loop()
        } else {
            Err(ParserError::UnexpectedToken(self.peek().clone()))
        }
    }

    fn block(&mut self) -> ExprResult {
        let mut stmts = vec![];
        let mut return_expr = None;

        while !self.check(TokenKind::RightCurly) && !self.is_at_end() {
            let expr_or_stmt = self.declaration()?;
            match expr_or_stmt {
                Stmt::Expr {
                    expr,
                    has_semicolon: false,
                } => {
                    return_expr = Some(expr);
                    break;
                }
                _ => {
                    stmts.push(expr_or_stmt);
                }
            }
        }

        self.consume(
            TokenKind::RightCurly,
            ParserError::UnclosedBlock(self.peek().clone()),
        )?;

        wrap_expr(Expr::Block { stmts, return_expr })
    }

    fn if_expr(&mut self) -> ExprResult {
        let condition = self.expression()?;

        self.consume(
            TokenKind::LeftCurly,
            ParserError::UnexpectedToken(self.peek().clone()),
        )?;

        let then_branch = self.block()?;

        let else_branch = if self.matches([TokenKind::Else]) {
            self.consume(
                TokenKind::LeftCurly,
                ParserError::UnexpectedToken(self.peek().clone()),
            )?;
            Some(self.block()?)
        } else {
            None
        };

        wrap_expr(Expr::If {
            condition,
            then_branch,
            else_branch,
        })
    }

    fn while_loop(&mut self) -> ExprResult {
        let condition = self.expression()?;

        self.consume(
            TokenKind::LeftCurly,
            ParserError::UnexpectedToken(self.peek().clone()),
        )?;

        let body = self.block()?;

        wrap_expr(Expr::While { condition, body })
    }

    fn for_loop(&mut self) -> ExprResult {
        let variable = self.primary()?;

        match variable.as_ref() {
            Expr::Variable(_) => {}
            _ => return Err(ParserError::UnexpectedToken(self.previous().clone())),
        };

        self.consume(
            TokenKind::In,
            ParserError::UnexpectedToken(self.peek().clone()),
        )?;

        let iterable = self.range()?;

        self.consume(
            TokenKind::LeftCurly,
            ParserError::UnexpectedToken(self.peek().clone()),
        )?;

        let body = self.block()?;

        wrap_expr(Expr::For {
            variable,
            iterable,
            body,
        })
    }

    fn return_expr(&mut self) -> ExprResult {
        if self.matches([TokenKind::Return]) {
            let keyword = self.previous().clone();
            let value = self.assignment();
            let value = match value {
                Ok(val) => Some(val),
                Err(ParserError::ExpectedExpression(_)) => None,
                _ => return value,
            };

            wrap_expr(Expr::Return { keyword, value })
        } else {
            self.assignment()
        }
    }

    fn assignment(&mut self) -> ExprResult {
        let expr = self.range()?;

        if !self.matches([TokenKind::Equal]) {
            return Ok(expr);
        }

        let equals = self.previous().clone();
        let value = self.assignment()?;

        if let Expr::Variable(name) = *expr {
            wrap_expr(Expr::Assignment { name, value })
        } else if let Expr::FieldAccess { object, name } = *expr {
            wrap_expr(Expr::FieldSet {
                object,
                name,
                value,
            })
        } else {
            Err(ParserError::InvalidAssignmentTarget(equals.clone()))
        }
    }

    fn range(&mut self) -> ExprResult {
        let mut expr = self.or()?;

        if self.matches([TokenKind::DotDot]) {
            let operator = self.previous().clone();
            let right = self.or()?;

            expr = Box::new(Expr::Binary {
                left: expr,
                operator,
                right,
            });
        }

        Ok(expr)
    }

    fn or(&mut self) -> ExprResult {
        let mut expr = self.and()?;

        while self.matches([TokenKind::Or]) {
            let operator = self.previous().clone();
            let right = self.and()?;

            expr = Box::new(Expr::Logical {
                left: expr,
                operator,
                right,
            })
        }

        Ok(expr)
    }

    fn and(&mut self) -> ExprResult {
        let mut expr = self.equality()?;

        while self.matches([TokenKind::And]) {
            let operator = self.previous().clone();
            let right = self.equality()?;

            expr = Box::new(Expr::Logical {
                left: expr,
                operator,
                right,
            })
        }

        Ok(expr)
    }

    fn equality(&mut self) -> ExprResult {
        let mut expr = self.comparison()?;

        while self.matches([TokenKind::BangEqual, TokenKind::EqualEqual]) {
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

        while self.matches([
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

        while self.matches([TokenKind::Plus, TokenKind::Minus]) {
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

        while self.matches([TokenKind::Star, TokenKind::Slash]) {
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
        if self.matches([TokenKind::Bang, TokenKind::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            wrap_expr(Expr::Unary { operator, right })
        } else {
            self.call()
        }
    }

    fn call(&mut self) -> ExprResult {
        let mut expr = self.primary()?;

        loop {
            if self.matches([TokenKind::LeftParen]) {
                expr = self.finish_call(expr)?;
            } else if self.matches([TokenKind::Dot]) {
                let name = self
                    .consume(
                        TokenKind::Identifier,
                        ParserError::UnexpectedToken(self.peek().clone()),
                    )?
                    .clone();
                expr = Box::new(Expr::FieldAccess { object: expr, name });
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn finish_call(&mut self, callee: BoxExpr) -> ExprResult {
        let mut args = vec![];
        if !self.check(TokenKind::RightParen) {
            loop {
                if args.len() >= 255 {
                    return Err(ParserError::UnexpectedToken(self.peek().clone()));
                }
                args.push(self.expression()?);
                if !self.matches([TokenKind::Comma]) {
                    break;
                }
            }
        }

        let paren = self
            .consume(
                TokenKind::RightParen,
                ParserError::UnexpectedToken(self.peek().clone()),
            )?
            .clone();

        wrap_expr(Expr::Call {
            callee,
            paren,
            args,
        })
    }

    fn primary(&mut self) -> ExprResult {
        if self.matches([TokenKind::Identifier]) {
            let prev = self.previous();
            return wrap_expr(Expr::Variable(prev.clone()));
        }

        if self.matches([TokenKind::Literal]) {
            let prev = self.previous();
            let literal_expr = match prev.literal.clone() {
                LiteralValue::String(s) => Ok(Expr::Literal(LiteralExpr::String(s))),
                LiteralValue::Float(f) => Ok(Expr::Literal(LiteralExpr::Float(f))),
                LiteralValue::Integer(n) => Ok(Expr::Literal(LiteralExpr::Integer(n))),
                _ => Err(ParserError::InvalidLiteral(prev.clone())),
            }?;
            return wrap_expr(literal_expr);
        }

        if self.matches([TokenKind::True, TokenKind::False]) {
            let literal_expr = match self.previous().kind {
                TokenKind::True => Ok(Expr::Literal(LiteralExpr::Bool(true))),
                TokenKind::False => Ok(Expr::Literal(LiteralExpr::Bool(false))),
                _ => Err(ParserError::InvalidLiteral(self.previous().clone())),
            }?;
            return wrap_expr(literal_expr);
        }

        if self.matches([TokenKind::LeftParen]) {
            let expr = self.expression()?;
            if self.advance().kind != TokenKind::RightParen {
                return Err(ParserError::ParenthesisNotClosed(self.previous().clone()));
            }
            return wrap_expr(Expr::Group(expr));
        }

        Err(ParserError::ExpectedExpression(self.peek().clone()))
    }

    fn matches<const N: usize>(&mut self, kinds: [TokenKind; N]) -> bool {
        for kind in kinds {
            if self.check(kind) {
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
        if self.check(token) {
            Ok(self.advance())
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

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().kind == TokenKind::Semicolon {
                return;
            }
            match self.peek().kind {
                TokenKind::Class
                | TokenKind::Fn
                | TokenKind::Var
                | TokenKind::For
                | TokenKind::If
                | TokenKind::While
                | TokenKind::Print
                | TokenKind::Return => return,
                _ => {}
            }
            self.advance();
        }
    }

    fn err_missing_semi(&self) -> ParserError {
        ParserError::MissingSemicolon(self.previous().clone())
    }
}
