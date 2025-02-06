use define_macro::define;

use crate::token::Token;

define! {

    enum statement ->   exprStmt(ExprStmt);


    struct exprStmt -> expr(Expression);


    enum expression ->  literal(Literal)
                        | ident(Token)
                        | call(Call)
                        | unary(Unary)
                        | binary(Binary)
                        | grouping(Box<Expression>);

    struct binary -> left(Box<Expression>), operator(Token), right(Box<Expression>);
    struct unary -> operator(Token), expr(Box<Expression>);
    struct call -> callee(Box<Expression>), args(Vec<Expression>);
    enum literal -> boolean(bool) | number(f64) | str(String) | null;
}
