use std::collections::HashMap;

use anyhow::bail;

use crate::{
    error::syntax_error,
    grammar::{BlockStmt, Declaration, Expression, LetDecl, Statement},
    interpreter::Interpreter,
    token::Token,
};

pub struct Resolver {
    interp: Interpreter,
    scopes: Vec<HashMap<String, bool>>,
}

impl Resolver {
    pub fn new(interp: Interpreter) -> Self {
        Self {
            interp,
            scopes: Vec::new(),
        }
    }

    fn resolve_declaration(&mut self, decl: &Declaration) -> anyhow::Result<()> {
        match decl {
            Declaration::StmtDecl(stmt_decl) => todo!(),
            Declaration::LetDecl(let_decl) => self.resolve_let_declaration(let_decl),
            Declaration::FnDecl(fn_decl) => todo!(),
        }
    }

    fn resolve_let_declaration(&mut self, let_decl: &LetDecl) -> anyhow::Result<()> {
        self.declare(&let_decl.ident);
        if let Some(init) = &let_decl.init {
            self.resolve_expression(init)?;
        }
        self.define(&let_decl.ident);
        Ok(())
    }

    fn resolve_statement(&mut self, stmt: &Statement) -> anyhow::Result<()> {
        match stmt {
            Statement::ExprStmt(expr_stmt) => todo!(),
            Statement::BlockStmt(block_stmt) => self.resolve_block_stmt(block_stmt),
            Statement::IfStmt(if_stmt) => todo!(),
            Statement::WhileStmt(while_stmt) => todo!(),
            Statement::ReturnStmt(return_stmt) => todo!(),
        }
    }

    fn resolve_block_stmt(&mut self, block_stmt: &BlockStmt) -> anyhow::Result<()> {
        self.begin_scope();
        block_stmt
            .stmts
            .iter()
            .try_for_each(|stmt| self.resolve_declaration(stmt))?;
        self.end_scope();
        Ok(())
    }

    fn resolve_expression(&mut self, expr: &Expression) -> anyhow::Result<()> {
        match expr {
            Expression::Literal(literal) => todo!(),
            Expression::Var(token) => self.resolve_var_expression(token),
            Expression::Call(call) => todo!(),
            Expression::Unary(unary) => todo!(),
            Expression::Binary(binary) => todo!(),
            Expression::Logical(logical) => todo!(),
            Expression::Range(range) => todo!(),
            Expression::Grouping(expression) => todo!(),
            Expression::Assignment(assignment) => todo!(),
        }
    }

    fn resolve_var_expression(&mut self, var: &Token) -> anyhow::Result<()> {
        if !self.scopes.is_empty() && self.scopes.last().unwrap().get(&var.lexeme) == Some(&false) {
            bail!(syntax_error(
                &var.line,
                "Can't read local variable in its own initializer"
            ));
        }

        self.resolve_local(var)?;
        Ok(())
    }

    fn resolve_local(&mut self, var: &Token) -> anyhow::Result<()> {
        todo!()
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare(&mut self, ident: &Token) {
        if self.scopes.is_empty() {
            return;
        }

        self.scopes
            .last_mut()
            .unwrap()
            .insert(ident.lexeme.clone(), false);
    }

    fn define(&mut self, ident: &Token) {
        if self.scopes.is_empty() {
            return;
        }

        self.scopes
            .last_mut()
            .unwrap()
            .insert(ident.lexeme.clone(), true);
    }
}
