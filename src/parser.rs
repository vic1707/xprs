/* Clippy Config */
#![allow(clippy::std_instead_of_core)]
/* Built-in imports */
use core::str;
use std::collections::HashMap;
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

#[derive(Debug, Default, PartialEq)]
#[non_exhaustive]
pub struct Parser {
    ctx: HashMap<String, f64>,
    fn_ctx: HashMap<String, fn(f64) -> f64>,
}

impl Parser {
    #[must_use]
    #[inline]
    pub const fn new_with_ctx(
        ctx: HashMap<String, f64>,
        fn_ctx: HashMap<String, fn(f64) -> f64>,
    ) -> Self {
        Self { ctx, fn_ctx }
    }

    #[must_use]
    #[inline]
    pub const fn ctx(&self) -> &HashMap<String, f64> {
        &self.ctx
    }

    #[must_use]
    #[inline]
    pub fn ctx_mut(&mut self) -> &mut HashMap<String, f64> {
        &mut self.ctx
    }

    #[must_use]
    #[inline]
    pub const fn fn_ctx(&self) -> &HashMap<String, fn(f64) -> f64> {
        &self.fn_ctx
    }

    #[must_use]
    #[inline]
    pub fn fn_ctx_mut(&mut self) -> &mut HashMap<String, fn(f64) -> f64> {
        &mut self.fn_ctx
    }

    #[inline]
    pub fn parse<'a>(
        &'a self,
        input: &'a str,
    ) -> Result<Element<'a>, ParseError> {
        ParserImpl::new(input, &self.ctx, &self.fn_ctx).parse()
    }
}

struct ParserImpl<'a> {
    input: &'a [u8],
    cursor: usize,
    ctx: &'a HashMap<String, f64>,
    fn_ctx: &'a HashMap<String, fn(f64) -> f64>,
}

impl<'a> ParserImpl<'a> {
    #[inline]
    #[must_use]
    pub const fn new(
        input: &'a str,
        ctx: &'a HashMap<String, f64>,
        fn_ctx: &'a HashMap<String, fn(f64) -> f64>,
    ) -> Self {
        Self {
            input: input.as_bytes(),
            cursor: 0,
            ctx,
            fn_ctx,
        }
    }

    #[inline]
    pub fn parse(&mut self) -> Result<Element<'a>, ParseError> {
        let root = self.element(NO_PERCEDENCE)?;
        if let Some(&tok) = self.next() {
            yeet!(ParseError::new_unexpected_token(self, tok));
        }
        Ok(root)
    }

    fn element(
        &mut self,
        precedence: usize,
    ) -> Result<Element<'a>, ParseError> {
        let mut el = self.atom()?;

        while let Some((op, op_precedence)) =
            self.get_operator_infos(&el, precedence)
        {
            let rhs = self.element(op_precedence)?;
            el = Element::BinOp(Box::new(BinOp::new(op, el, rhs)));
        }

        Ok(el)
    }

    fn atom(&mut self) -> Result<Element<'a>, ParseError> {
        let Some(&next) = self.next() else {
            yeet!(ParseError::new_unexpected_end_of_expression(self));
        };
        let atom = match next {
            /* Number */
            b'0'..=b'9' | b'.' => self.parse_number()?,
            /* Identifier */
            b'a'..=b'z' => self.parse_identifier()?,
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
                    yeet!(ParseError::new_expected_token(self, b')'));
                }
                self.cursor += 1;
                el
            },
            /* Errors */
            b')' => yeet!(ParseError::new_unexpected_token(self, b')')),
            tok => {
                yeet!(ParseError::new_illegal_character(self, tok));
            },
        };

        Ok(atom)
    }

    fn parse_identifier(&mut self) -> Result<Element<'a>, ParseError> {
        let name = self.take_while(u8::is_ascii_lowercase);

        // checks for contexts or built-in functions
        // else defaults to variable
        let ident = self
            .ctx
            .get(name)
            .map(|&value| Identifier::Constant(value))
            .or_else(|| {
                self.fn_ctx
                    .get(name)
                    .map(|&func| Identifier::Function(func))
            })
            .unwrap_or_else(|| name.into());

        let el = match ident {
            Identifier::Constant(val) => Element::Number(val),
            Identifier::Variable(var) => Element::Variable(var),
            Identifier::Function(func) if Some(&b'(') == self.next() => {
                let el = self.element(FunctionCall::PRECEDENCE)?;
                Element::Function(Box::new(FunctionCall::new(func, el)))
            },
            Identifier::Function(_) => {
                yeet!(ParseError::new_expected_token(self, b'('))
            },
        };
        Ok(el)
    }

    fn parse_number(&mut self) -> Result<Element<'a>, ParseError> {
        let ident = self
            .take_while(|&ch| matches!(ch, b'0'..=b'9' | b'.' | b'e' | b'E'));

        let num = ident
            .parse()
            .map_err(|_err| ParseError::new_malformed_number(self, ident))?;

        Ok(Element::Number(num))
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

impl ParserImpl<'_> {
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

    fn get_operator_infos(
        &mut self,
        current_atom: &Element<'_>,
        precedence: usize,
    ) -> Option<(Operator, usize)> {
        let current_byte = *self.next()?;
        // check for binary operator
        if let Ok(op) = Operator::try_from(current_byte) {
            let op_p = BinOp::precedence(&op);
            if op_p <= precedence {
                return None;
            }
            self.cursor += 1;
            return Some((op, op_p));
        }

        #[allow(clippy::items_after_statements)]
        const TIMES_INFOS: (Operator, usize) =
            (Operator::Times, BinOp::IMPLICIT_MULTIPLICATION_PRECEDENCE);
        match current_byte {
            // if multiplication precedence is lower than current precedence
            // we now we don't need implicit multiplication
            _ if TIMES_INFOS.1 <= precedence => None,
            // if it's an identifier or an opening parenthesis
            // we can consider its an implicit multiplication
            b'a'..=b'z' | b'(' => Some(TIMES_INFOS),
            // if it's a number implicit multiplication is
            // only possible if previous atom isn't a number
            b'0'..=b'9' if !matches!(*current_atom, Element::Number(_)) => {
                Some(TIMES_INFOS)
            },
            _ => None,
        }
    }
}

#[derive(Debug, Eq, PartialEq, thiserror::Error, Diagnostic)]
#[error("{kind}")]
pub struct ParseError {
    kind: ErrorKind,
    #[label("here")]
    span: SourceSpan,
    #[source_code]
    src: String,
}

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
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

impl ParseError {
    fn new_unexpected_end_of_expression(parser: &ParserImpl) -> Self {
        Self {
            kind: ErrorKind::UnexpectedEndOfExpression,
            // - 1 because we want to point to the last character
            // (without it we would point to a `None` value)
            span: (parser.cursor - 1).into(),
            src: trust_me!(str::from_utf8_unchecked(parser.input)).to_owned(),
        }
    }

    fn new_unexpected_token(parser: &ParserImpl, tok: u8) -> Self {
        Self {
            kind: ErrorKind::UnexpectedToken(char::from(tok)),
            span: parser.cursor.into(),
            src: trust_me!(str::from_utf8_unchecked(parser.input)).to_owned(),
        }
    }

    fn new_malformed_number(parser: &ParserImpl, ident: &str) -> Self {
        let num_len = ident.len();
        Self {
            kind: ErrorKind::MalformedNumber(ident.to_owned()),
            span: (parser.cursor - num_len, num_len).into(),
            src: trust_me!(str::from_utf8_unchecked(parser.input)).to_owned(),
        }
    }

    fn new_illegal_character(parser: &ParserImpl, tok: u8) -> Self {
        Self {
            kind: ErrorKind::IllegalCharacter(char::from(tok)),
            span: parser.cursor.into(),
            src: trust_me!(str::from_utf8_unchecked(parser.input)).to_owned(),
        }
    }

    fn new_expected_token(parser: &ParserImpl, tok: u8) -> Self {
        Self {
            kind: ErrorKind::ExpectedToken(char::from(tok)),
            span: parser.cursor.into(),
            src: trust_me!(str::from_utf8_unchecked(parser.input)).to_owned(),
        }
    }
}
