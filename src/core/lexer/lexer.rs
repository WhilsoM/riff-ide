use crate::core::lexer::token::{Token, TokenKind};

pub struct Lexer<'a> {
    src: &'a str,
    chars: Vec<(usize, char)>,
    pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        let chars: Vec<_> = src.char_indices().collect();
        Self { src, chars, pos: 0 }
    }

    pub fn lex(mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while self.pos < self.chars.len() {
            let token = self.next_token();
            tokens.push(token);
        }

        tokens
    }

    fn next_token(&mut self) -> Token {
        let start_byte = self.chars[self.pos].0;
        let c = self.chars[self.pos].1;

        // 1. whitespace
        if c.is_whitespace() {
            self.consume_while(|_, ch| ch.is_whitespace());
            let end_byte = if self.pos == self.chars.len() {
                self.src.len()
            } else {
                self.chars[self.pos].0
            };
            return Token {
                kind: TokenKind::Whitespace,
                range: start_byte..end_byte,
            };
        }

        // 2. line comment //
        if c == '/' && self.peek_char() == Some('/') {
            self.pos += 2; // пропускаем //
            self.consume_while(|_, ch| ch != '\n');
            let end_byte = if self.pos == self.chars.len() {
                self.src.len()
            } else {
                self.chars[self.pos].0
            };
            return Token {
                kind: TokenKind::Comment,
                range: start_byte..end_byte,
            };
        }

        // 3. string literal "..."
        if c == '"' {
            self.pos += 1; // пропускаем открывающую "
            self.consume_while(|_, ch| ch != '"'); // простая строка, не поддерживает экранирование здесь
            if self.pos < self.chars.len() && self.chars[self.pos].1 == '"' {
                self.pos += 1; // закрывающая "
            }
            let end_byte = if self.pos == self.chars.len() {
                self.src.len()
            } else {
                self.chars[self.pos].0
            };
            return Token {
                kind: TokenKind::String,
                range: start_byte..end_byte,
            };
        }

        // 4. number
        if c.is_ascii_digit() {
            self.consume_while(|_, ch| ch.is_ascii_digit());
            let end_byte = if self.pos == self.chars.len() {
                self.src.len()
            } else {
                self.chars[self.pos].0
            };
            return Token {
                kind: TokenKind::Number,
                range: start_byte..end_byte,
            };
        }

        // 5. unknown
        self.pos += 1;
        let end_byte = if self.pos == self.chars.len() {
            self.src.len()
        } else {
            self.chars[self.pos].0
        };
        Token {
            kind: TokenKind::Unknown,
            range: start_byte..end_byte,
        }
    }

    fn peek_char(&self) -> Option<char> {
        if self.pos + 1 < self.chars.len() {
            Some(self.chars[self.pos + 1].1)
        } else {
            None
        }
    }

    fn consume_while<F>(&mut self, mut f: F)
    where
        F: FnMut(usize, char) -> bool,
    {
        while self.pos < self.chars.len() {
            let (i, c) = self.chars[self.pos];
            if !f(i, c) {
                break;
            }
            self.pos += 1;
        }
    }
}
