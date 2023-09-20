/* Built-in imports */
use core::str;
/* Crate imports */
use crate::{
    token::{Operator, Token},
    trust_me::trust_me,
};

pub struct Lexer<'a> {
    input: &'a [u8],
    position: usize,
}

impl<'a> Lexer<'a> {
    #[inline]
    #[must_use]
    pub const fn new(input: &'a str) -> Self {
        Self {
            input: input.as_bytes(),
            position: 0,
        }
    }
}

#[allow(clippy::missing_trait_methods)]
impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token<'a>, &'static str>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        while (self.input.get(self.position)?).is_ascii_whitespace() {
            self.position += 1;
        }

        let token: Token = match *(self.input.get(self.position)?) {
            // b'!' => Token::Operator(Operator::Factorial),
            b'+' => Token::Operator(Operator::Plus),
            b'-' => Token::Operator(Operator::Minus),
            b'*' => Token::Operator(Operator::Times),
            b'/' => Token::Operator(Operator::Divide),
            b'^' => Token::Operator(Operator::Power),
            b'%' => Token::Operator(Operator::Modulo),
            b'(' => Token::LParen,
            b')' => Token::RParen,
            b'0'..=b'9' | b'.' => {
                let begin = self.position;
                while self.input.get(self.position).is_some_and(|&ch| {
                    matches!(ch, b'0'..=b'9' | b'.' | b'e' | b'E')
                }) {
                    self.position += 1;
                }

                #[allow(clippy::unreachable)]
                let Some(indent) = self.input.get(begin..self.position) else {
                    unreachable!()
                };

                let Ok(num) =
                    trust_me! { str::from_utf8_unchecked(indent) }.parse()
                else {
                    return Some(Err("Failed to parse number"));
                };

                // early return is necessary here so we don't
                // advance past the last character of the
                // current identifier
                return Some(Ok(Token::Number(num)));
            },
            b'a'..=b'z' => {
                let begin = self.position;
                while self
                    .input
                    .get(self.position)
                    .is_some_and(u8::is_ascii_lowercase)
                {
                    self.position += 1;
                }

                #[allow(clippy::unreachable)]
                let Some(indent_bytes) = self.input.get(begin..self.position) else {
                    unreachable!()
                };

                let indent =
                    trust_me! { str::from_utf8_unchecked(indent_bytes) };

                // early return is necessary here so we don't
                // advance past the last character of the
                // current identifier
                return Some(Ok(Token::Identifier(indent.into())));
            },
            _ => return Some(Err("Illegal character")),
        };

        self.position += 1;
        Some(Ok(token))
    }
}
