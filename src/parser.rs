use anyhow::bail;

use crate::{
    grammar::{
        Assignment, Binary, BlockStmt, Call, Declaration, ExprStmt, Expression, IfStmt, LetDecl,
        Literal, Logical, Range, Statement, StmtDecl, Unary, WhileStmt,
    },
    runtime_error, syntax_error,
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

    pub fn parse(&mut self) -> anyhow::Result<Vec<Declaration>> {
        let mut declarations = Vec::new();
        while !self.finished() {
            match self.parse_declaration() {
                Ok(s) => declarations.push(s),
                Err(e) => {
                    self.errors.push_str(&e.to_string());
                    self.synchronize()
                }
            }
        }

        if self.errors.is_empty() {
            return Ok(declarations);
        }

        bail!(self.errors.clone())
    }

    fn parse_declaration(&mut self) -> anyhow::Result<Declaration> {
        if let TokenType::Let = self.peek().ty {
            return self.parse_let_declaration();
        }

        let stmt = self.parse_statment()?;
        Ok(Declaration::StmtDecl(StmtDecl::new(stmt)))
    }

    fn parse_let_declaration(&mut self) -> anyhow::Result<Declaration> {
        let let_token = self.next_token();
        let line = let_token.line;

        let ident = self
            .expect(
                TokenType::Identifier,
                "expected identifier after let declaration",
                line,
            )?
            .clone();

        let mut init = None;
        if let TokenType::Equal = self.peek().ty {
            self.next_token();
            init = Some(self.parse_expression()?);
        }

        self.expect(
            TokenType::Semicolon,
            "Expect ';' after variable declaration",
            line,
        )?;

        Ok(Declaration::LetDecl(LetDecl::new(ident, init)))
    }

    fn parse_statment(&mut self) -> anyhow::Result<Statement> {
        if let TokenType::LeftBrace = self.peek().ty {
            return self.parse_block_statement();
        }

        if let TokenType::If = self.peek().ty {
            return self.parse_if_statement();
        }

        if let TokenType::While = self.peek().ty {
            return self.parse_while_statement();
        }

        if let TokenType::For = self.peek().ty {
            return self.parse_for_statement();
        }

        let expr = self.parse_expression()?;
        self.expect(
            TokenType::Semicolon,
            "Expected ';' after expression",
            self.peek_previous().line,
        )?;
        Ok(Statement::ExprStmt(ExprStmt::new(expr)))
    }

    fn parse_for_statement(&mut self) -> anyhow::Result<Statement> {
        let for_token = self.next_token();
        let line = for_token.line;

        let variable = self
            .expect(
                TokenType::Identifier,
                "Expected identifier after 'for' keyword",
                line,
            )?
            .clone();
        self.expect(
            TokenType::In,
            "Expected 'in' keyword after identifier in for loop declaration",
            line,
        )?;

        let range = self.parse_range()?;
        let body = self.parse_block_statement()?;

        // Extract start and end values from the range expression.
        let (start, end) = match range {
            Expression::Range(r) => match (*r.left, *r.right) {
                (
                    Expression::Literal(Literal::Number(start)),
                    Expression::Literal(Literal::Number(end)),
                ) => (start, end),
                _ => bail!(syntax_error(
                    &line,
                    "Expected range operands to evaluate to a number"
                )),
            },
            _ => bail!(syntax_error(
                &line,
                "Expected range expression (a..b) in for loop declaration"
            )),
        };

        // Create the loop variable declaration: let variable = start;
        let var_decl = Declaration::LetDecl(LetDecl::new(
            variable.clone(),
            Some(Expression::Literal(Literal::Number(start))),
        ));

        // Build the loop condition: variable < end.
        let condition = Expression::Binary(Binary::new(
            Box::new(Expression::Var(variable.clone())),
            Token::new("<".to_string(), TokenType::Less, line),
            Box::new(Expression::Literal(Literal::Number(end))),
        ));

        // Build the increment statement: variable = variable + 1.
        let increment_expr = Expression::Binary(Binary::new(
            Box::new(Expression::Var(variable.clone())),
            Token::new("+".to_string(), TokenType::Plus, line),
            Box::new(Expression::Literal(Literal::Number(1.0))),
        ));
        let assign =
            Expression::Assignment(Assignment::new(variable.clone(), Box::new(increment_expr)));
        let iteration = Statement::ExprStmt(ExprStmt::new(assign));

        // Ensure the loop body is a block and append the iteration statement.
        let while_body = match body {
            Statement::BlockStmt(mut block) => {
                block
                    .stmts
                    .push(Declaration::StmtDecl(StmtDecl::new(iteration)));
                Statement::BlockStmt(block)
            }
            _ => bail!(syntax_error(
                &line,
                "Expected block after for loop declaration"
            )),
        };

        // Construct the while loop using the condition and modified body.
        let while_stmt = Declaration::StmtDecl(StmtDecl::new(Statement::WhileStmt(
            WhileStmt::new(condition, Box::new(while_body)),
        )));

        // Return the desugared for-loop as a block containing the variable declaration and while loop.
        Ok(Statement::BlockStmt(BlockStmt::new(vec![
            var_decl, while_stmt,
        ])))
    }

    fn parse_while_statement(&mut self) -> anyhow::Result<Statement> {
        let _while_token = self.next_token();
        let condition = self.parse_expression()?;
        let body = Box::new(self.parse_block_statement()?);
        Ok(Statement::WhileStmt(WhileStmt::new(condition, body)))
    }

    fn parse_if_statement(&mut self) -> anyhow::Result<Statement> {
        let _if_token = self.next_token();

        let condition = self.parse_expression()?;
        let if_branch = Box::new(self.parse_block_statement()?);

        let mut else_branch = None;
        if let TokenType::Else = self.peek().ty {
            self.next_token();
            else_branch = Some(Box::new(self.parse_block_statement()?));
        }

        Ok(Statement::IfStmt(IfStmt::new(
            condition,
            if_branch,
            else_branch,
        )))
    }

    fn parse_block_statement(&mut self) -> anyhow::Result<Statement> {
        let left_brace_token = self.expect(
            TokenType::LeftBrace,
            "Expected '{' at begining of block",
            self.peek_previous().line,
        )?;
        let line = left_brace_token.line;

        let mut stmts = Vec::new();

        while self.current < self.tokens.len()
            && !matches!(self.tokens[self.current].ty, TokenType::RightBrace)
        {
            stmts.push(self.parse_declaration()?);
        }

        if self.current >= self.tokens.len() {
            return Err(anyhow::anyhow!(runtime_error(&line, "Unclosed block")));
        }

        self.expect(
            TokenType::RightBrace,
            "Expected '}' at the end of scope",
            line,
        )?;

        Ok(Statement::BlockStmt(BlockStmt::new(stmts)))
    }

    fn parse_expression(&mut self) -> anyhow::Result<Expression> {
        self.parse_assignment()
    }

    fn parse_assignment(&mut self) -> anyhow::Result<Expression> {
        let expr = self.parse_range()?;

        if let TokenType::Equal = self.peek().ty {
            //consumens the '=' token
            let equals = self.next_token().clone();
            let value = self.parse_assignment()?;

            if let Expression::Var(v) = expr {
                let ident = v;
                return Ok(Expression::Assignment(Assignment::new(
                    ident,
                    Box::new(value),
                )));
            }

            bail!(syntax_error(&equals.line, "Invalid assigment target"))
        }

        Ok(expr)
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
            left = Expression::Logical(Logical::new(Box::new(left), op, Box::new(right)))
        }

        Ok(left)
    }

    fn parse_and(&mut self) -> anyhow::Result<Expression> {
        let mut left = self.parse_equality()?;

        while let TokenType::And = self.peek().ty {
            let op = self.next_token().clone();
            let right = self.parse_equality()?;
            left = Expression::Logical(Logical::new(Box::new(left), op, Box::new(right)))
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
                let token = self.next_token().clone();
                callee = self.parse_fn_args(callee, token)?;
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
            TokenType::Identifier => Ok(Expression::Var(primary)),
            TokenType::LeftParen => {
                let expr = self.parse_expression()?;
                self.expect(
                    TokenType::RightParen,
                    "Expected ')' after expression",
                    primary.line,
                )?;
                Ok(Expression::Grouping(Box::new(expr)))
            }
            _ => bail!(syntax_error(
                &primary.line,
                &format!("Expected expression. Found {:?}", primary.lexeme)
            )),
        }
    }

    fn parse_fn_params(&mut self) -> anyhow::Result<Vec<Token>> {
        let left_paren = self
            .expect(
                TokenType::LeftParen,
                "Expected '(' before function parameters",
                self.peek_previous().line,
            )?
            .clone();

        let mut params = Vec::new();
        while !matches!(self.peek().ty, TokenType::RightParen) {
            let arg = self
                .expect(
                    TokenType::Identifier,
                    "Expected parameters identifiers",
                    left_paren.line,
                )?
                .clone();
            params.push(arg);
            if let TokenType::Comma = self.peek().ty {
                self.next_token();
            }
        }

        self.expect(
            TokenType::RightParen,
            "Expected ')' after function parameters",
            self.peek_previous().line,
        )?;
        Ok(params)
    }

    fn parse_fn_args(&mut self, e: Expression, paren_token: Token) -> anyhow::Result<Expression> {
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
        Ok(Expression::Call(Call::new(Box::new(e), paren_token, args)))
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
        self.tokens
            .get(self.current)
            .unwrap_or_else(|| self.tokens.last().expect("Token list is empty"))
    }

    fn peek_previous(&self) -> &Token {
        if self.current == 0 {
            panic!("No previous token available");
        }
        &self.tokens[self.current - 1]
    }

    fn next_token(&mut self) -> &Token {
        if self.finished() {
            return self.peek();
        }
        let token = &self.tokens[self.current];
        self.current += 1;
        token
    }
}
