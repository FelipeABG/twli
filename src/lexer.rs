use crate::token::{Token, TokenType};
use anyhow::bail;

pub struct Lexer {
    source: String,
    current: usize,
    start: usize,
    line: usize,
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self {
            source,
            current: 0,
            start: 0,
            line: 1,
            tokens: Vec::new(),
        }
    }

    pub fn tokenize(&mut self) -> anyhow::Result<Vec<&Token>> {
        while !self.finished() {
            self.start = self.current;
            self.scan_token()?;
        }

        Ok(self.tokens.iter().collect())
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
            _ if char.is_digit(10) => self.add_number_token(),
            ' ' | '\r' | '\t' => (),
            '\n' => self.line += 1,
            _ => bail!(format!("[line {}] Unexpected token '{}'", self.line, char)),
        }

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
        self.source.chars().nth(self.current).unwrap()
    }

    fn peek1(&self) -> char {
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
