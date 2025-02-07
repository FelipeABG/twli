use anyhow::bail;

use crate::{
    grammar::{Binary, Call, Expression, Item, Literal, Logical, Range, Unary},
    lox::Object,
    runtime_error,
    token::TokenType,
};

#[derive(Default)]
pub struct Interpreter {}

impl Interpreter {
    pub fn interpret(&mut self, itens: Vec<Item>) -> anyhow::Result<()> {
        todo!()
    }

    fn evaluate_expression(&mut self, expr: &Expression) -> anyhow::Result<Object> {
        match expr {
            Expression::Literal(literal) => self.evaluate_literal(literal),
            Expression::Ident(token) => todo!(),
            Expression::Call(call) => self.evaluate_call(call),
            Expression::Range(range) => self.evaluate_range(range),
            Expression::Unary(unary) => self.evaluate_unary(unary),
            Expression::Binary(binary) => self.evaluate_binary(binary),
            Expression::Logical(logical) => self.evaluate_logical(logical),
            Expression::Grouping(expression) => self.evaluate_expression(expression),
        }
    }

    fn evaluate_range(&mut self, range: &Range) -> anyhow::Result<Object> {
        todo!()
    }

    fn evaluate_logical(&mut self, logical: &Logical) -> anyhow::Result<Object> {
        let left = self.evaluate_expression(&logical.left)?;

        if let TokenType::Or = logical.operator.ty {
            if left.truthiness() {
                return Ok(left);
            }
        } else {
            if !left.truthiness() {
                return Ok(left);
            }
        }

        self.evaluate_expression(&logical.right)
    }

    fn evaluate_binary(&mut self, binary: &Binary) -> anyhow::Result<Object> {
        let left = self.evaluate_expression(&binary.left)?;
        let right = self.evaluate_expression(&binary.right)?;
        let line = binary.operator.line;

        match binary.operator.ty {
            TokenType::BangEqual => Ok(Object::Boolean(left != right)),
            TokenType::EqualEqual => Ok(Object::Boolean(left == right)),
            TokenType::Greater => Ok(Object::Boolean(left > right)),
            TokenType::GreaterEqual => Ok(Object::Boolean(left >= right)),
            TokenType::LessEqual => Ok(Object::Boolean(left <= right)),
            TokenType::Less => Ok(Object::Boolean(left < right)),
            TokenType::Minus => Ok(Object::Number(
                left.expect_number(line)? - right.expect_number(line)?,
            )),
            TokenType::Slash => Ok(Object::Number(
                left.expect_number(line)? / right.expect_number(line)?,
            )),
            TokenType::Star => Ok(Object::Number(
                left.expect_number(line)? * right.expect_number(line)?,
            )),
            TokenType::Plus => {
                if let (Ok(n1), Ok(n2)) = (left.expect_number(line), right.expect_number(line)) {
                    return Ok(Object::Number(n1 + n2));
                }

                if let (Ok(n1), Ok(n2)) = (left.expect_string(line), right.expect_string(line)) {
                    return Ok(Object::Str(n1 + &n2));
                }

                bail!(runtime_error(&line, "Expected 'string' or 'number'"))
            }
            _ => bail!(runtime_error(
                &binary.operator.line,
                "Unexpected operator of binary operation"
            )),
        }
    }

    fn evaluate_unary(&mut self, unary: &Unary) -> anyhow::Result<Object> {
        let value = self.evaluate_expression(&unary.expr)?;

        match unary.operator.ty {
            TokenType::Bang => Ok(Object::Boolean(!value.truthiness())),
            TokenType::Minus => {
                if let Object::Number(n) = value {
                    return Ok(Object::Number(-n));
                }

                bail!(runtime_error(
                    &unary.operator.line,
                    "Expected number to be followed by '-'"
                ))
            }
            _ => bail!(runtime_error(
                &unary.operator.line,
                "Unexpected operator. Expected '!' or '-'"
            )),
        }
    }

    fn evaluate_call(&mut self, call: &Call) -> anyhow::Result<Object> {
        todo!()
    }

    fn evaluate_literal(&mut self, lit: &Literal) -> anyhow::Result<Object> {
        Ok(match lit {
            Literal::Boolean(b) => Object::Boolean(*b),
            Literal::Number(n) => Object::Number(*n),
            Literal::Str(s) => Object::Str(s.clone()),
            Literal::Null => Object::Null,
        })
    }
}
