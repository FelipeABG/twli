use define_macro::define;

use crate::token::Token;

define! {
    enum declaration -> stmtDecl(StmtDecl)
                        | letDecl(LetDecl);

    struct stmtDecl -> stmt(Statement);
    struct letDecl -> ident(Token), init(Option<Expression>);

    enum statement -> exprStmt(ExprStmt)
                        | blockStmt(BlockStmt)
                        | ifStmt(IfStmt)
                        | whileStmt(WhileStmt);

    struct whileStmt -> condition(Expression), body(Box<Statement>);
    struct ifStmt -> condition(Expression), if_branch(Box<Statement>), else_branch(Option<Box<Statement>>);
    struct exprStmt -> expr(Expression);
    struct BlockStmt -> stmts(Vec<Declaration>);

    enum expression ->  literal(Literal)
                        | var(Token)
                        | call(Call)
                        | unary(Unary)
                        | binary(Binary)
                        | logical(Logical)
                        | range(Range)
                        | grouping(Box<Expression>)
                        | assignment(Assignment);

    struct assignment -> ident(Token), expr(Box<Expression>);
    struct range -> left(Box<Expression>), right(Box<Expression>);
    struct logical ->left(Box<Expression>), operator(Token), right(Box<Expression>);
    struct binary -> left(Box<Expression>), operator(Token), right(Box<Expression>);
    struct unary -> operator(Token), expr(Box<Expression>);
    struct call -> callee(Box<Expression>), args(Vec<Expression>);
    enum literal -> boolean(bool) | number(f64) | str(String) | null;
}
