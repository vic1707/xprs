/* Clippy Config */
#![allow(clippy::std_instead_of_core)]
/* Built-in imports */
use core::str;
/* Crate imports */
use crate::{
    element::{BinOp, Element, FunctionCall, UnOp},
    macros::{trust_me, yeet},
    token::{Identifier, Operator},
};
/* Dependencies imports */
use miette::{Diagnostic, SourceSpan};
/* Constants */
pub const NO_PERCEDENCE: usize = 0;

pub struct Parser<'a> {
    input: &'a [u8],
    cursor: usize,
}

impl<'a> Parser<'a> {
    #[inline]
    #[must_use]
    pub const fn new(input: &'a str) -> Self {
        Self {
            input: input.as_bytes(),
            cursor: 0,
        }
    }

    #[inline]
    pub fn parse(&mut self) -> Result<Element<'a>, Error> {
        let root = self.element(NO_PERCEDENCE)?;
        if let Some(&tok) = self.next() {
            yeet!(Error {
                kind: ErrorKind::UnexpectedToken(char::from(tok)),
                span: self.cursor.into(),
                src: trust_me!(str::from_utf8_unchecked(self.input)).to_owned(),
            });
        }
        Ok(root)
    }

    fn element(&mut self, precedence: usize) -> Result<Element<'a>, Error> {
        let mut el = self.atom()?;

        while let Some(op) = self.get_operator(&el, precedence) {
            let current_precedence = BinOp::precedence(&op);
            let rhs = self.element(current_precedence)?;
            el = Element::BinOp(Box::new(BinOp::new(op, el, rhs)));
        }

        Ok(el)
    }

    fn atom(&mut self) -> Result<Element<'a>, Error> {
        let Some(next) = self.next() else {
            yeet!(Error {
                kind: ErrorKind::UnexpectedEndOfExpression,
                // -1 because we want to point to the last character (currently pointing to `None`)
                span: (self.cursor - 1).into(),
                src: trust_me!(str::from_utf8_unchecked(self.input)).to_owned(),
            });
        };
        let atom = match *next {
            /* Number */
            b'0'..=b'9' | b'.' => Element::Number(self.parse_number()?),
            /* Unary expression */
            op @ (b'+' | b'-') => {
                self.cursor += 1;
                #[allow(clippy::unreachable)]
                let operator = match op {
                    b'+' => Operator::Plus,
                    b'-' => Operator::Minus,
                    _ => unreachable!(),
                };
                let operand = self.element(UnOp::PRECEDENCE)?;
                Element::UnOp(Box::new(UnOp::new(operator, operand)))
            },
            /* Parenthesis */
            b'(' => {
                self.cursor += 1;
                let el = self.element(NO_PERCEDENCE)?;
                if self.next() != Some(&b')') {
                    yeet!(Error {
                        kind: ErrorKind::ExpectedToken(')'),
                        // -1 because we want to point to where the `)` should be
                        // not where the next token is
                        span: (self.cursor - 1).into(),
                        src: trust_me!(str::from_utf8_unchecked(self.input))
                            .to_owned(),
                    });
                }
                self.cursor += 1;
                el
            },
            /* Identifier */
            b'a'..=b'z' => match self.parse_identifier() {
                Identifier::Constant(val) => Element::Number(val),
                Identifier::Variable(var) => Element::Variable(var),
                Identifier::Function(func) if Some(&b'(') == self.next() => {
                    let el = self.element(FunctionCall::PRECEDENCE)?;
                    Element::Function(Box::new(FunctionCall::new(func, el)))
                },
                Identifier::Function(_) => yeet!(Error {
                    kind: ErrorKind::ExpectedToken('('),
                    span: self.cursor.into(),
                    src: trust_me!(str::from_utf8_unchecked(self.input))
                        .to_owned(),
                }),
            },
            tok => {
                yeet!(Error {
                    kind: ErrorKind::IllegalCharacter(char::from(tok)),
                    span: self.cursor.into(),
                    src: trust_me!(str::from_utf8_unchecked(self.input))
                        .to_owned(),
                });
            },
        };

        Ok(atom)
    }

    fn parse_identifier(&mut self) -> Identifier<'a> {
        self.take_while(u8::is_ascii_lowercase).into()
    }

    #[inline]
    fn take_while(&mut self, predicate: fn(&u8) -> bool) -> &'a str {
        let start = self.cursor;
        self.skip_while(predicate);
        let end = self.cursor;
        trust_me!(
            #[allow(clippy::indexing_slicing)]
            str::from_utf8_unchecked(&self.input[start..end])
        )
    }
}

impl Parser<'_> {
    #[inline]
    fn skip_while(&mut self, predicate: fn(&u8) -> bool) {
        while self.current().is_some_and(predicate) {
            self.cursor += 1;
        }
    }

    #[inline]
    fn current(&self) -> Option<&u8> {
        self.input.get(self.cursor)
    }

    fn next(&mut self) -> Option<&u8> {
        self.skip_while(u8::is_ascii_whitespace);
        self.current()
    }

    fn parse_number(&mut self) -> Result<f64, Error> {
        let begin = self.cursor;
        let ident = self
            .take_while(|&ch| matches!(ch, b'0'..=b'9' | b'.' | b'e' | b'E'));

        let num = ident.parse().map_err(|_err| Error {
            kind: ErrorKind::MalformedNumber(ident.to_owned()),
            span: (begin..self.cursor).into(),
            src: trust_me!(str::from_utf8_unchecked(self.input)).to_owned(),
        })?;

        Ok(num)
    }

    fn get_operator(
        &mut self,
        current_atom: &Element<'_>,
        precedence: usize,
    ) -> Option<Operator> {
        let current_byte = *self.next()?;
        // check for binary operator
        if let Ok(op) = Operator::try_from(current_byte) {
            if BinOp::precedence(&op) <= precedence {
                return None;
            }
            self.cursor += 1;
            return Some(op);
        }
        // check for implicit multiplication
        if BinOp::precedence(&Operator::Times) > precedence {
            return match current_byte {
                // if it's an identifier or an opening parenthesis
                // we can consider its an implicit multiplication
                b'a'..=b'z' | b'(' => Some(Operator::Times),
                // if it's a number implicit multiplication is
                // only possible if previous atom isn't a number
                b'0'..=b'9' if !matches!(*current_atom, Element::Number(_)) => {
                    Some(Operator::Times)
                },
                _ => None,
            };
        }
        None
    }
}

#[allow(clippy::error_impl_error)]
#[derive(Debug, Eq, PartialEq, thiserror::Error, Diagnostic)]
#[error("{kind}")]
pub struct Error {
    kind: ErrorKind,
    #[label("here")]
    span: SourceSpan,
    #[source_code]
    src: String,
}

#[derive(Debug, Eq, PartialEq, thiserror::Error, Diagnostic)]
#[non_exhaustive]
pub enum ErrorKind {
    #[error("Unexpected end of expression")]
    UnexpectedEndOfExpression,
    #[error("Unexpected token: `{0}`")]
    UnexpectedToken(char),
    #[error("Malforned number: `{0}`")]
    MalformedNumber(String),
    #[error("Illegal character: `{0}`")]
    IllegalCharacter(char),
    #[error("Expected token: `{0}`")]
    ExpectedToken(char),
}
