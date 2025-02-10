use anyhow::bail;

use crate::{
    grammar::{Binary, Call, Expression, Literal, Unary},
    runtime::Object,
    runtime_error,
    token::TokenType,
};

struct Interpreter {}

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
            Expression::Logical(logical) => todo!(),
            Expression::Range(range) => todo!(),
            Expression::Grouping(expression) => todo!(),
            Expression::Assignment(assignment) => todo!(),
        }
    }

    fn eval_binary(&mut self, binary: &Binary) -> anyhow::Result<Object> {
        let left = self.eval_expression(&binary.left)?;
        let right = self.eval_expression(&binary.right)?;
        let line = &binary.operator.line;
    }

    fn eval_unary(&mut self, unary: &Unary) -> anyhow::Result<Object> {
        let value = self.eval_expression(&unary.expr)?;
        match unary.operator.ty {
            TokenType::Bang => Ok(Object::Boolean(!value.thrutiness())),
            TokenType::Minus => Ok(Object::Number(-value.expect_number()?)),
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
