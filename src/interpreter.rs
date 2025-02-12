use std::{cell::RefCell, cmp::Ordering, rc::Rc};

use anyhow::{anyhow, bail};

use crate::{
    env::Environment,
    grammar::{
        Assignment, Binary, BlockStmt, Call, Declaration, ExprStmt, Expression, IfStmt, LetDecl,
        Literal, Logical, Range, Statement, Unary,
    },
    runtime::Object,
    runtime_error,
    token::TokenType,
};

pub struct Interpreter {
    global: Rc<RefCell<Environment>>,
    current: Rc<RefCell<Environment>>,
}

impl Interpreter {
    pub fn new() -> Self {
        let global = Rc::new(RefCell::new(Environment::new(None)));
        Self {
            global: Rc::clone(&global),
            current: Rc::clone(&global),
        }
    }

    pub fn interpret(&mut self, ast: Vec<Declaration>) -> anyhow::Result<()> {
        for stmt in ast.iter() {
            self.register_declaration(stmt)?
        }

        Ok(())
    }

    fn register_declaration(&mut self, decl: &Declaration) -> anyhow::Result<()> {
        match decl {
            Declaration::StmtDecl(stmt_decl) => self.exec_statement(&stmt_decl.stmt),
            Declaration::LetDecl(let_decl) => self.register_let_declaration(let_decl),
        }
    }

    fn register_let_declaration(&mut self, let_decl: &LetDecl) -> anyhow::Result<()> {
        match &let_decl.init {
            Some(i) => {
                let init = self.eval_expression(&i)?;
                Ok(RefCell::borrow_mut(&self.global).define(let_decl.ident.lexeme.clone(), init))
            }
            None => Ok(RefCell::borrow_mut(&self.global)
                .define(let_decl.ident.lexeme.clone(), Object::Null)),
        }
    }

    fn exec_statement(&mut self, stmt: &Statement) -> anyhow::Result<()> {
        match stmt {
            Statement::ExprStmt(expr_stmt) => self.exec_expression_statement(expr_stmt),
            Statement::BlockStmt(block_stmt) => self.exec_block_statement(block_stmt),
            Statement::IfStmt(if_stmt) => self.exec_if_statement(if_stmt),
        }
    }

    fn exec_if_statement(&mut self, if_stmt: &IfStmt) -> anyhow::Result<()> {
        let condition = self.eval_expression(&if_stmt.condition)?;

        if condition.thrutiness() {
            self.exec_statement(&if_stmt.if_branch)?;
        } else {
            if let Some(else_branch) = &if_stmt.else_branch {
                self.exec_statement(&else_branch)?;
            }
        }
        Ok(())
    }

    fn exec_block_statement(&mut self, block_stmt: &BlockStmt) -> anyhow::Result<()> {
        let previous = Rc::clone(&self.current);
        self.current = Rc::new(RefCell::new(Environment::new(Some(Rc::clone(
            &self.current,
        )))));

        for decl in block_stmt.stmts.iter() {
            self.register_declaration(decl)?
        }

        self.current = previous;
        Ok(())
    }

    fn exec_expression_statement(&mut self, expr_stmt: &ExprStmt) -> anyhow::Result<()> {
        self.eval_expression(&expr_stmt.expr)?;
        Ok(())
    }

    fn eval_expression(&mut self, expr: &Expression) -> anyhow::Result<Object> {
        match expr {
            Expression::Literal(literal) => self.eval_literal(literal),
            Expression::Var(token) => RefCell::borrow_mut(&self.current).get(&token.lexeme),
            Expression::Call(call) => self.eval_call(call),
            Expression::Unary(unary) => self.eval_unary(unary),
            Expression::Binary(binary) => self.eval_binary(binary),
            Expression::Logical(logical) => self.eval_logical(logical),
            Expression::Range(range) => self.eval_range(range),
            Expression::Grouping(expression) => self.eval_expression(expression),
            Expression::Assignment(assignment) => self.eval_assignment(assignment),
        }
    }

    fn eval_assignment(&mut self, assignment: &Assignment) -> anyhow::Result<Object> {
        let value = self.eval_expression(&assignment.expr)?;
        let line = &assignment.ident.line;
        RefCell::borrow_mut(&self.global)
            .assign(&assignment.ident.lexeme, value.clone())
            .map_err(|e| anyhow!(runtime_error(line, &e.to_string())))?;
        Ok(value)
    }

    fn eval_range(&mut self, range: &Range) -> anyhow::Result<Object> {
        todo!()
    }

    fn eval_logical(&mut self, logical: &Logical) -> anyhow::Result<Object> {
        todo!()
    }

    fn eval_binary(&mut self, binary: &Binary) -> anyhow::Result<Object> {
        let left = self.eval_expression(&binary.left)?;
        let right = self.eval_expression(&binary.right)?;
        let line = &binary.operator.line;

        match binary.operator.ty {
            TokenType::EqualEqual => Ok(Object::Boolean(left == right)),
            TokenType::BangEqual => Ok(Object::Boolean(left != right)),
            TokenType::Minus => {
                (left - right).map_err(|e| anyhow::anyhow!(runtime_error(line, &e.to_string())))
            }
            TokenType::Star => {
                (left * right).map_err(|e| anyhow::anyhow!(runtime_error(line, &e.to_string())))
            }
            TokenType::Slash => {
                (left / right).map_err(|e| anyhow::anyhow!(runtime_error(line, &e.to_string())))
            }
            TokenType::Plus => {
                (left + right).map_err(|e| anyhow::anyhow!(runtime_error(line, &e.to_string())))
            }
            TokenType::Greater => match left.partial_cmp(&right) {
                Some(a) => if let Ordering::Greater = a {
                    Ok(Object::Boolean(true))
                }else {
                    Ok(Object::Boolean(false))
                },
                None => bail!(runtime_error(line, "Ordering operators can only be used when both operands are 'string' or 'number'")),
            },
            TokenType::GreaterEqual => match left.partial_cmp(&right) {
                Some(a) => if let Ordering::Greater | Ordering::Equal= a {
                    Ok(Object::Boolean(true))
                }else {
                    Ok(Object::Boolean(false))
                },
                None => bail!(runtime_error(line, "Ordering operators can only be used when both operands are 'string' or 'number'")),
            },
            TokenType::LessEqual => match left.partial_cmp(&right) {
                Some(a) => if let Ordering::Less | Ordering::Equal= a {
                    Ok(Object::Boolean(true))
                }else {
                    Ok(Object::Boolean(false))
                },
                None => bail!(runtime_error(line, "Ordering operators can only be used when both operands are 'string' or 'number'")),
            },
            TokenType::Less => match left.partial_cmp(&right) {
                Some(a) => if let Ordering::Less = a {
                    Ok(Object::Boolean(true))
                }else {
                    Ok(Object::Boolean(false))
                },
                None => bail!(runtime_error(line, "Ordering operators can only be used when both operands are 'string' or 'number'")),
            },
            _ => bail!(runtime_error(line, "Unexpected binary operator")),
        }
    }

    fn eval_unary(&mut self, unary: &Unary) -> anyhow::Result<Object> {
        let value = self.eval_expression(&unary.expr)?;
        let line = &unary.operator.line;
        match unary.operator.ty {
            TokenType::Bang => Ok(Object::Boolean(!value.thrutiness())),
            TokenType::Minus => Ok(Object::Number(-value.expect_number(line)?)),
            _ => bail!(runtime_error(
                &unary.operator.line,
                "Expected '-' or '!' in unary operations"
            )),
        }
    }

    fn eval_call(&mut self, call: &Call) -> anyhow::Result<Object> {
        todo!()
    }

    fn eval_literal(&mut self, literal: &Literal) -> anyhow::Result<Object> {
        Ok(match literal {
            Literal::Boolean(b) => Object::Boolean(*b),
            Literal::Number(n) => Object::Number(*n),
            Literal::Str(s) => Object::Str(s.clone()),
            Literal::Null => Object::Null,
        })
    }
}
