use define_macro::define;

use crate::token::Token;

define! {

    enum item -> fnItem(FnItem);

    struct fnItem -> fun(FnDecl);

    enum declaration -> fnDecl(FnDecl)
                        | letDecl(LetDecl)
                        | stmtDecl(StmtDecl);

    struct stmtDecl -> stmt(Statement);
    struct fnDecl -> ident(Token), params(Vec<Token>), body(Statement);
    struct letDecl -> ident(Token), init(Option<Expression>);

    enum statement ->   exprStmt(ExprStmt)
                        | returnStmt(ReturnStmt)
                        | whileStmt(WhileStmt)
                        | fotStmt(ForStmt)
                        | ifStmt(IfStmt)
                        | block(Vec<Declaration>);

    struct ifStmt -> condition(Expression), if_branch(Box<Statement>), else_branch(Option<Box<Statement>>);
    struct forStmt -> ident(Token), range(Expression), body(Box<Statement>);
    struct whileStmt -> condition(Expression), body(Box<Statement>);
    struct returnStmt -> expr(Option<Expression>);
    struct exprStmt -> expr(Expression);


    enum expression ->  literal(Literal)
                        | ident(Token)
                        | call(Call)
                        | range(Range)
                        | unary(Unary)
                        | binary(Binary)
                        | grouping(Box<Expression>);

    struct binary -> left(Box<Expression>), operator(Token), right(Box<Expression>);
    struct unary -> operator(Token), expr(Box<Expression>);
    struct range -> left(Box<Expression>), right(Box<Expression>);
    struct call -> callee(Box<Expression>), args(Vec<Expression>);
    enum literal -> boolean(bool) | number(f64) | str(String) | null;
}
