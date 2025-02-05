use define_macro::define;

use crate::token::Token;

define! {
    enum expression ->  literal(Literal)
                        | unary(Unary)
                        | binary(Binary)
                        | grouping(Box<Expression>);

    struct binary -> left(Box<Expression>), operator(Token), right(Box<Expression>);
    struct unary -> operator(Token), expr(Box<Expression>);
    enum literal -> boolean(bool) | number(f64) | str(String) | null;
}
