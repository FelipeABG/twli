use std::collections::HashMap;

use once_cell::sync::Lazy;

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
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

pub static KEYWORDS: Lazy<HashMap<String, TokenType>> = Lazy::new(|| {
    let mut keywords = HashMap::new();
    keywords.insert("let".to_string(), TokenType::Let);
    keywords.insert("fn".to_string(), TokenType::Fn);
    keywords.insert("while".to_string(), TokenType::While);
    keywords.insert("for".to_string(), TokenType::For);
    keywords.insert("in".to_string(), TokenType::In);
    keywords.insert("and".to_string(), TokenType::And);
    keywords.insert("or".to_string(), TokenType::Or);
    keywords.insert("if".to_string(), TokenType::If);
    keywords.insert("else".to_string(), TokenType::Else);
    keywords.insert("null".to_string(), TokenType::Null);
    keywords.insert("return".to_string(), TokenType::Return);
    keywords.insert("true".to_string(), TokenType::True);
    keywords.insert("false".to_string(), TokenType::False);
    keywords.insert("this".to_string(), TokenType::This);
    keywords.insert("super".to_string(), TokenType::Super);
    keywords.insert("class".to_string(), TokenType::Class);
    keywords
});
