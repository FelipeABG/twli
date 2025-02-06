use define_macro::define;

use crate::token::Token;

define! {

    enum statement ->   exprStmt(ExprStmt)
                        | letStmt(LetStmt)
                        | returnStmt(ReturnStmt);


    struct returnStmt -> expr(Option<Expression>);
    struct exprStmt -> expr(Expression);
    struct letStmt -> ident(Token), init(Option<Expression>);


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
