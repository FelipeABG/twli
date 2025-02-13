use crate::{
    error::syntax_error,
    token::{Token, TokenType, KEYWORDS},
};
use anyhow::bail;

pub struct Lexer {
    source: String,
    current: usize,
    start: usize,
    line: usize,
    tokens: Vec<Token>,
    errors: String,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self {
            source,
            current: 0,
            start: 0,
            line: 1,
            tokens: Vec::new(),
            errors: "".to_string(),
        }
    }

    pub fn tokenize(&mut self) -> anyhow::Result<Vec<Token>> {
        self.reset();
        while !self.finished() {
            self.start = self.current;
            if let Err(e) = self.scan_token() {
                self.errors.push_str(&e.to_string());
            }
        }

        if self.errors.is_empty() {
            return Ok(self.tokens.clone());
        }

        bail!(self.errors.clone())
    }

    fn reset(&mut self) {
        self.current = 0;
        self.start = 0;
        self.line = 1;
        self.tokens = Vec::new();
    }

    fn scan_token(&mut self) -> anyhow::Result<()> {
        let char = self.next_char();
        match char {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                if self.complement('=') {
                    self.add_token(TokenType::BangEqual);
                } else {
                    self.add_token(TokenType::Bang);
                }
            }
            '=' => {
                if self.complement('=') {
                    self.add_token(TokenType::EqualEqual);
                } else {
                    self.add_token(TokenType::Equal);
                }
            }
            '>' => {
                if self.complement('=') {
                    self.add_token(TokenType::GreaterEqual);
                } else {
                    self.add_token(TokenType::Greater);
                }
            }
            '<' => {
                if self.complement('=') {
                    self.add_token(TokenType::LessEqual);
                } else {
                    self.add_token(TokenType::Less);
                }
            }
            '.' => {
                if self.complement('.') {
                    self.add_token(TokenType::DotDot);
                } else {
                    self.add_token(TokenType::Dot);
                }
            }
            '"' => self.add_string_token()?,
            _ if char.is_digit(10) => self.add_number_token(),
            _ if char.is_alphabetic() || char == '_' => self.add_identifier_token(),
            ' ' | '\r' | '\t' => (),
            '\n' => self.line += 1,
            _ => bail!(syntax_error(
                &self.line,
                &format!("Unexpected Token '{}'", char)
            )),
        }

        Ok(())
    }

    fn add_identifier_token(&mut self) {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.next_char();
        }

        let something = &self.source[self.start..self.current];
        if let Some(kw) = KEYWORDS.get(something) {
            self.add_token(kw.clone());
            return;
        }

        self.add_token(TokenType::Identifier);
    }

    fn add_string_token(&mut self) -> anyhow::Result<()> {
        while self.peek() != '"' && !self.finished() {
            self.next_char();
        }

        if self.finished() {
            bail!(syntax_error(&self.line, "Unterminated string"))
        }

        //consumes the '"'
        self.next_char();

        let string = &self.source[self.start + 1..self.current - 1];
        self.add_token(TokenType::String(String::from(string)));
        Ok(())
    }

    fn add_number_token(&mut self) {
        while self.peek().is_digit(10) {
            self.next_char();
        }

        if self.peek() == '.' && self.peek1().is_digit(10) {
            self.next_char();
            while self.peek().is_digit(10) {
                self.next_char();
            }
        }

        let number = self.source[self.start..self.current]
            .parse::<f64>()
            .unwrap();
        self.add_token(TokenType::Number(number));
    }

    fn complement(&mut self, c: char) -> bool {
        if self.peek() == c {
            self.current += 1;
            return true;
        }

        false
    }

    fn add_token(&mut self, ty: TokenType) {
        let lexeme = self.source[self.start..self.current].to_string();
        let token = Token::new(lexeme, ty, self.line);
        self.tokens.push(token);
    }

    fn peek(&self) -> char {
        if self.finished() {
            return '\0';
        }
        self.source.chars().nth(self.current).unwrap()
    }

    fn peek1(&self) -> char {
        if self.current + 1 == self.source.len() {
            return '\0';
        }
        self.source.chars().nth(self.current + 1).unwrap()
    }

    fn next_char(&mut self) -> char {
        let current_char = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        current_char
    }

    fn finished(&self) -> bool {
        self.current >= self.source.len()
    }
}
