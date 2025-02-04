#[derive(Debug)]
pub struct Token {
    pub lexeme: String,
    pub ty: TokenType,
    pub line: usize,
}

impl Token {
    pub fn new(lexeme: String, ty: TokenType, line: usize) -> Self {
        Self { lexeme, ty, line }
    }
}

#[derive(Debug)]
pub enum TokenType {
    //single char Tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    //single or double char tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    DotDot,

    // literals
    Identifier,
    String(String),
    Number(f64),

    //keywords
    And,
    Class,
    Else,
    False,
    Fn,
    For,
    If,
    In,
    Null,
    Or,
    Return,
    Super,
    This,
    True,
    Let,
    While,
}
