use anyhow::bail;

use crate::{
    grammar::{Binary, Expression, Literal, Unary},
    syntax_error,
    token::{Token, TokenType},
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> anyhow::Result<Expression> {
        self.parse_expression()
    }

    fn parse_expression(&mut self) -> anyhow::Result<Expression> {
        self.parse_or()
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
            let expr = self.parse_literal()?;
            return Ok(Expression::Unary(Unary::new(op, Box::new(expr))));
        }

        self.parse_literal()
    }

    fn parse_literal(&mut self) -> anyhow::Result<Expression> {
        let lit = self.next_token().clone();
        match lit.ty {
            TokenType::Number(n) => Ok(Expression::Literal(Literal::Number(n))),
            TokenType::String(s) => Ok(Expression::Literal(Literal::Str(s))),
            TokenType::False => Ok(Expression::Literal(Literal::Boolean(false))),
            TokenType::True => Ok(Expression::Literal(Literal::Boolean(true))),
            TokenType::Null => Ok(Expression::Literal(Literal::Null)),
            TokenType::LeftParen => {
                let expr = self.parse_expression()?;
                self.expect(
                    TokenType::RightParen,
                    "Expected ')' after expression",
                    &lit.line,
                )?;
                Ok(Expression::Grouping(Box::new(expr)))
            }
            _ => bail!(syntax_error(&lit.line, "Expected expression.")),
        }
    }

    fn expect(&mut self, ty: TokenType, msg: &str, line: &usize) -> anyhow::Result<&Token> {
        if self.peek().ty == ty {
            return Ok(self.next_token());
        }

        bail!(syntax_error(line, msg))
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

    fn next_token(&mut self) -> &Token {
        let result = &self.tokens[self.current];
        self.current += 1;
        result
    }
}
