use std::cmp::Ordering;

use anyhow::bail;

use crate::{
    grammar::{Assignment, Binary, Call, Expression, Literal, Logical, Range, Unary},
    runtime::Object,
    runtime_error,
    token::TokenType,
};

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn interpret(&mut self, ast: Vec<Expression>) -> anyhow::Result<()> {
        for stmt in ast.iter() {
            println!("{}", self.eval_expression(stmt)?)
        }

        Ok(())
    }

    fn eval_expression(&mut self, expr: &Expression) -> anyhow::Result<Object> {
        match expr {
            Expression::Literal(literal) => self.eval_literal(literal),
            Expression::Var(token) => todo!(),
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
        todo!()
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
