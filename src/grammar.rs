use define_macro::define;

use crate::token::Token;

define! {
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
