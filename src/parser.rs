use anyhow::bail;

use crate::{
    grammar::{Binary, Call, ExprStmt, Expression, LetStmt, Literal, Range, Statement, Unary},
    syntax_error,
    token::{Token, TokenType},
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    errors: String,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
            errors: "".to_string(),
        }
    }

    pub fn parse(&mut self) -> anyhow::Result<Vec<Statement>> {
        let mut stmts = Vec::new();
        while !self.finished() {
            match self.parse_statement() {
                Ok(s) => stmts.push(s),
                Err(e) => {
                    self.errors.push_str(&e.to_string());
                    self.synchronize()
                }
            }
        }

        if self.errors.is_empty() {
            return Ok(stmts);
        }

        bail!(self.errors.clone())
    }

    fn parse_statement(&mut self) -> anyhow::Result<Statement> {
        if let TokenType::Let = self.peek().ty {
            return self.parse_let_statement();
        }
        self.parse_expression_statement()
    }

    fn parse_let_statement(&mut self) -> anyhow::Result<Statement> {
        let let_kw = self.next_token().clone();
        let ident = self
            .expect(
                TokenType::Identifier,
                &format!(
                    "Expected identifier after let declaration. Found {:?}",
                    self.peek().lexeme,
                ),
                let_kw.line,
            )?
            .clone();

        let mut init = None;
        if let TokenType::Equal = self.peek().ty {
            self.next_token();
            init = Some(Box::new(self.parse_expression()?));
        }

        self.expect(
            TokenType::Semicolon,
            "Expected ';' after let declaration",
            let_kw.line,
        )?;
        Ok(Statement::LetStmt(LetStmt::new(ident, init)))
    }

    fn parse_expression_statement(&mut self) -> anyhow::Result<Statement> {
        let expr = self.parse_expression()?;
        self.expect(
            TokenType::Semicolon,
            "Expected ';' after expression",
            self.peek_previous().line,
        )?;
        Ok(Statement::ExprStmt(ExprStmt::new(expr)))
    }

    fn parse_expression(&mut self) -> anyhow::Result<Expression> {
        self.parse_range()
    }

    fn parse_range(&mut self) -> anyhow::Result<Expression> {
        let left = self.parse_or()?;

        if let TokenType::DotDot = self.peek().ty {
            self.next_token();
            let right = self.parse_or()?;
            return Ok(Expression::Range(Range::new(
                Box::new(left),
                Box::new(right),
            )));
        }

        Ok(left)
    }

    fn parse_or(&mut self) -> anyhow::Result<Expression> {
        let mut left = self.parse_and()?;

        while let TokenType::Or = self.peek().ty {
            let op = self.next_token().clone();
            let right = self.parse_or()?;
            left = Expression::Binary(Binary::new(Box::new(left), op, Box::new(right)))
        }

        Ok(left)
    }

    fn parse_and(&mut self) -> anyhow::Result<Expression> {
        let mut left = self.parse_equality()?;

        while let TokenType::And = self.peek().ty {
            let op = self.next_token().clone();
            let right = self.parse_equality()?;
            left = Expression::Binary(Binary::new(Box::new(left), op, Box::new(right)))
        }

        Ok(left)
    }

    fn parse_equality(&mut self) -> anyhow::Result<Expression> {
        let mut left = self.parse_comparison()?;

        while let TokenType::EqualEqual | TokenType::BangEqual = self.peek().ty {
            let op = self.next_token().clone();
            let right = self.parse_comparison()?;
            left = Expression::Binary(Binary::new(Box::new(left), op, Box::new(right)))
        }

        Ok(left)
    }

    fn parse_comparison(&mut self) -> anyhow::Result<Expression> {
        let mut left = self.parse_term()?;

        while let TokenType::Greater
        | TokenType::GreaterEqual
        | TokenType::Less
        | TokenType::LessEqual = self.peek().ty
        {
            let op = self.next_token().clone();
            let right = self.parse_term()?;
            left = Expression::Binary(Binary::new(Box::new(left), op, Box::new(right)))
        }

        Ok(left)
    }

    fn parse_term(&mut self) -> anyhow::Result<Expression> {
        let mut left = self.parse_factor()?;

        while let TokenType::Minus | TokenType::Plus = self.peek().ty {
            let op = self.next_token().clone();
            let right = self.parse_factor()?;
            left = Expression::Binary(Binary::new(Box::new(left), op, Box::new(right)))
        }

        Ok(left)
    }

    fn parse_factor(&mut self) -> anyhow::Result<Expression> {
        let mut left = self.parse_unary()?;

        while let TokenType::Star | TokenType::Slash = self.peek().ty {
            let op = self.next_token().clone();
            let right = self.parse_unary()?;
            left = Expression::Binary(Binary::new(Box::new(left), op, Box::new(right)))
        }

        Ok(left)
    }

    fn parse_unary(&mut self) -> anyhow::Result<Expression> {
        if matches!(self.peek().ty, TokenType::Minus | TokenType::Bang) {
            let op = self.next_token().clone();
            let expr = self.parse_primary()?;
            return Ok(Expression::Unary(Unary::new(op, Box::new(expr))));
        }

        self.parse_call()
    }

    fn parse_call(&mut self) -> anyhow::Result<Expression> {
        let mut callee = self.parse_primary()?;

        loop {
            if let TokenType::LeftParen = self.peek().ty {
                //consumes the '(' token
                self.next_token();
                callee = self.parse_fn_args(callee)?;
            } else {
                break;
            }
        }

        Ok(callee)
    }

    fn parse_primary(&mut self) -> anyhow::Result<Expression> {
        let primary = self.next_token().clone();
        match primary.ty {
            TokenType::Number(n) => Ok(Expression::Literal(Literal::Number(n))),
            TokenType::String(s) => Ok(Expression::Literal(Literal::Str(s))),
            TokenType::False => Ok(Expression::Literal(Literal::Boolean(false))),
            TokenType::True => Ok(Expression::Literal(Literal::Boolean(true))),
            TokenType::Null => Ok(Expression::Literal(Literal::Null)),
            TokenType::Identifier => Ok(Expression::Ident(primary)),
            TokenType::LeftParen => {
                let expr = self.parse_expression()?;
                self.expect(
                    TokenType::RightParen,
                    "Expected ')' after expression",
                    primary.line,
                )?;
                Ok(Expression::Grouping(Box::new(expr)))
            }
            _ => bail!(syntax_error(&primary.line, "Expected expression")),
        }
    }

    fn parse_fn_args(&mut self, e: Expression) -> anyhow::Result<Expression> {
        let mut args = Vec::new();
        while !matches!(self.peek().ty, TokenType::RightParen) {
            let arg = self.parse_expression()?;
            args.push(arg);
            if let TokenType::Comma = self.peek().ty {
                self.next_token();
            }
        }

        self.expect(
            TokenType::RightParen,
            "Expected ')' after function arguments",
            self.peek_previous().line,
        )?;
        Ok(Expression::Call(Call::new(Box::new(e), args)))
    }

    fn synchronize(&mut self) {
        self.next_token();

        while !self.finished() {
            if let TokenType::Semicolon = self.peek_previous().ty {
                return;
            }

            match self.peek().ty {
                TokenType::Class
                | TokenType::Let
                | TokenType::Fn
                | TokenType::For
                | TokenType::While
                | TokenType::If
                | TokenType::Return => return,
                _ => self.next_token(),
            };
        }
    }

    fn expect(&mut self, ty: TokenType, msg: &str, line: usize) -> anyhow::Result<&Token> {
        if self.peek().ty == ty {
            return Ok(self.next_token());
        }

        bail!(syntax_error(&line, msg))
    }

    fn finished(&self) -> bool {
        self.current >= self.tokens.len()
    }

    fn peek(&self) -> &Token {
        if self.finished() {
            return &self.tokens[self.current - 1];
        }
        &self.tokens[self.current]
    }

    fn peek_previous(&self) -> &Token {
        if self.finished() {
            return &self.tokens[self.current - 2];
        }
        &self.tokens[self.current - 1]
    }

    fn next_token(&mut self) -> &Token {
        let result = &self.tokens[self.current];
        self.current += 1;
        result
    }
}
