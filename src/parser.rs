/* Clippy Config */
#![allow(clippy::std_instead_of_core)]
/* Built-in imports */
use core::str;
use std::collections::HashSet;
/* Crate imports */
#[cfg(feature = "compile-time-optimizations")]
use crate::element::Simplify;
use crate::{
    context::Context,
    element::{BinOp, Element, FunctionCall, UnOp},
    token::{Identifier, Operator},
    utils::{
        macros::{trust_me, yeet},
        precedence,
    },
    xprs::Xprs,
};

#[derive(Debug, Default, PartialEq)]
pub struct Parser<'ctx> {
    ctx: Context<'ctx>,
}

impl<'ctx> Parser<'ctx> {
    #[inline]
    #[must_use]
    pub const fn new_with_ctx(ctx: Context<'ctx>) -> Self {
        Self { ctx }
    }

    #[inline]
    #[must_use]
    pub const fn ctx(&self) -> &Context {
        &self.ctx
    }

    #[inline]
    pub fn ctx_mut(&mut self) -> &mut Context<'ctx> {
        &mut self.ctx
    }

    #[inline]
    pub fn parse<'input>(
        &self,
        input: &'input str,
    ) -> Result<Xprs<'input>, ParseError> {
        let xprs = ParserImpl::parse(input, &self.ctx)?;

        // Check if no unknown variable was found
        if let Some(expected) = self.ctx.get_expected_vars() {
            if let Some(unknown_var) = xprs.vars.difference(expected).next() {
                yeet!(ParseError::new_variable_not_declared(
                    input,
                    unknown_var,
                    expected
                        .iter()
                        .map(|&str| str.to_owned())
                        .collect::<Vec<_>>()
                ))
            }
        }

        Ok(xprs)
    }
}

struct ParserImpl<'input, 'ctx> {
    input: &'input [u8],
    cursor: usize,
    ctx: &'ctx Context<'ctx>,
}

impl<'input, 'ctx> ParserImpl<'input, 'ctx> {
    const fn new(input: &'input str, ctx: &'ctx Context<'ctx>) -> Self {
        Self {
            input: input.as_bytes(),
            cursor: 0,
            ctx,
        }
    }

    pub fn parse(
        input: &'input str,
        ctx: &'ctx Context<'ctx>,
    ) -> Result<Xprs<'input>, ParseError> {
        let mut parser_impl = Self::new(input, ctx);

        let root = parser_impl.element(precedence::NO_PRECEDENCE)?;

        if let Some(&tok) = parser_impl.next_trim() {
            yeet!(ParseError::new_unexpected_token(&parser_impl, tok));
        }

        // TODO: find a better way to get variables
        // don't like the fact that we go through the whole tree
        // to find variables
        let mut vars: HashSet<&str> = HashSet::default();
        root.find_variables(&mut vars);

        Ok(Xprs { root, vars })
    }

    fn element(
        &mut self,
        precedence: usize,
    ) -> Result<Element<'input>, ParseError> {
        let mut el = self.atom()?;

        while let Some((op, op_precedence)) =
            self.get_operator_infos(&el, precedence)
        {
            let rhs = self.element(op_precedence)?;
            el = BinOp::new_element(op, el, rhs);
        }

        #[cfg(feature = "compile-time-optimizations")]
        {
            el = el.simplify();
        };

        Ok(el)
    }

    fn atom(&mut self) -> Result<Element<'input>, ParseError> {
        let Some(&next) = self.next_trim() else {
            yeet!(ParseError::new_unexpected_end_of_expression(self));
        };
        let atom = match next {
            /* Number */
            b'0'..=b'9' | b'.' => self.parse_number()?,
            /* Identifier */
            b'A'..=b'z' => self.parse_identifier()?,
            /* Unary expression */
            op @ (b'+' | b'-') => {
                self.cursor += 1;
                #[allow(clippy::unreachable)]
                let operator = match op {
                    b'+' => Operator::Plus,
                    b'-' => Operator::Minus,
                    _ => unreachable!(),
                };
                let operand = self.element(precedence::UNOP_PRECEDENCE)?;
                UnOp::new_element(operator, operand)
            },
            /* Parenthesis */
            b'(' => {
                self.cursor += 1;
                let el = self.element(precedence::NO_PRECEDENCE)?;
                self.assert_eq_consume(b')')?;
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

    fn parse_identifier(&mut self) -> Result<Element<'input>, ParseError> {
        let identifier_start = self.cursor;
        let name = self.take_while(
            |&ch| matches!(ch, b'_' | b'\'' | b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9'),
        );

        // checks for contexts or built-in functions
        // else defaults to variable
        let ident = self
            .ctx
            .get_var(name)
            .map(|&value| Identifier::Constant(value))
            .or_else(|| {
                self.ctx.get_func(name).copied().map(Identifier::Function)
            })
            .unwrap_or_else(|| Identifier::from_str(name));

        let el = match ident {
            Identifier::Constant(val) => Element::Number(val),
            Identifier::Variable(var) => Element::Variable(var),
            Identifier::Function(func) if self.consume_if_eq(b'(') => {
                // for now the minimum number of arguments is 1
                // self.parse_arguments() will fail if no argument is found
                let args = self.parse_arguments()?;
                self.assert_eq_consume(b')')?;
                if let Some(nb_args) = func.nb_args {
                    use std::cmp::Ordering::{Equal, Greater, Less};
                    match args.len().cmp(&nb_args.into()) {
                        Equal => (),
                        Less => {
                            yeet!(ParseError::new_too_few_arguments(
                                self,
                                nb_args,
                                args.len(),
                                identifier_start
                            ))
                        },
                        Greater => {
                            yeet!(ParseError::new_too_many_arguments(
                                self,
                                nb_args,
                                args.len(),
                                identifier_start
                            ))
                        },
                    }
                }
                FunctionCall::new_element(func, args)
            },
            Identifier::Function(_) => {
                yeet!(ParseError::new_expected_token(self, b'('))
            },
        };

        Ok(el)
    }

    fn parse_number(&mut self) -> Result<Element<'input>, ParseError> {
        let begin = self.cursor;
        self.skip_while(|&ch| matches!(ch, b'0'..=b'9' | b'.' | b'_'));
        // make sure to not mistake exponent (10^) with exponential (e = 2.71828..)
        let might_be_exponent = matches!(self.current(), Some(&b'e' | &b'E'));
        let is_exponent_with_sign = might_be_exponent
            && matches!(self.next(), Some(&b'+' | &b'-'))
            && matches!(self.next_at(2), Some(&(b'0'..=b'9')));
        let is_exponent =
            might_be_exponent && matches!(self.next(), Some(&(b'0'..=b'9')));

        self.cursor +=
            usize::from(is_exponent_with_sign) * 2 + usize::from(is_exponent);

        self.skip_while(u8::is_ascii_digit);
        let end = self.cursor;

        let ident = trust_me!(
            #[allow(clippy::indexing_slicing)]
            str::from_utf8_unchecked(&self.input[begin..end])
        );

        let num = ident.replace('_', "").parse().map_err(
            #[cold]
            |_err| ParseError::new_malformed_number(self, ident),
        )?;

        Ok(Element::Number(num))
    }

    fn parse_arguments(&mut self) -> Result<Vec<Element<'input>>, ParseError> {
        let mut args = Vec::new();

        loop {
            let arg = self.argument()?;
            args.push(arg);

            // expect either a comma or a closing parenthesis
            match self.next_trim() {
                Some(&b',') => self.cursor += 1,
                Some(&b')') => break,
                Some(&tok) => {
                    yeet!(ParseError::new_unexpected_token(self, tok))
                },
                None => {
                    yeet!(ParseError::new_unexpected_end_of_expression(self))
                },
            }
        }
        Ok(args)
    }

    fn argument(&mut self) -> Result<Element<'input>, ParseError> {
        self.element(precedence::NO_PRECEDENCE).map_err(
            #[cold]
            |err| match err.kind {
                ErrorKind::UnexpectedToken(_) => {
                    ParseError::new_missing_argument(self)
                },
                ErrorKind::UnexpectedEndOfExpression
                | ErrorKind::ExpectedToken(_)
                | ErrorKind::MalformedNumber(_)
                | ErrorKind::IllegalCharacter(_)
                | ErrorKind::VariableNotDeclared(_, _)
                | ErrorKind::TooFewArguments(_, _)
                | ErrorKind::TooManyArguments(_, _)
                | ErrorKind::MissingArgument => err,
            },
        )
    }

    fn take_while(&mut self, predicate: fn(&u8) -> bool) -> &'input str {
        let start = self.cursor;
        self.skip_while(predicate);
        let end = self.cursor;
        trust_me!(
            #[allow(clippy::indexing_slicing)]
            str::from_utf8_unchecked(&self.input[start..end])
        )
    }
}

impl ParserImpl<'_, '_> {
    fn skip_while(&mut self, predicate: fn(&u8) -> bool) {
        while self.current().is_some_and(predicate) {
            self.cursor += 1;
        }
    }

    fn consume_if_eq(&mut self, tok: u8) -> bool {
        let eq = self.next_trim() == Some(&tok);
        self.cursor += usize::from(eq);
        eq
    }

    fn current(&self) -> Option<&u8> {
        self.input.get(self.cursor)
    }

    fn next_trim(&mut self) -> Option<&u8> {
        self.skip_while(u8::is_ascii_whitespace);
        self.current()
    }

    fn next(&self) -> Option<&u8> {
        self.input.get(self.cursor + 1)
    }

    fn next_at(&self, offset: usize) -> Option<&u8> {
        self.input.get(self.cursor + offset)
    }

    fn get_operator_infos(
        &mut self,
        current_atom: &Element<'_>,
        precedence: usize,
    ) -> Option<(Operator, usize)> {
        use precedence::IMPLICIT_MULTIPLICATION_INFO;

        let current_byte = *self.next_trim()?;
        // check for binary operator
        if let Ok(op) = Operator::try_from(current_byte) {
            let op_p = precedence::get_for_op(&op);
            if op_p <= precedence {
                return None;
            }
            self.cursor += 1;
            return Some((op, op_p));
        }

        match current_byte {
            // if multiplication precedence is lower than current precedence
            // we now we don't need implicit multiplication
            _ if IMPLICIT_MULTIPLICATION_INFO.1 <= precedence => None,
            // if it's an identifier or an opening parenthesis
            // we can consider its an implicit multiplication
            b'a'..=b'z' | b'(' => Some(IMPLICIT_MULTIPLICATION_INFO),
            // if it's a number implicit multiplication is
            // only possible if previous atom isn't a number
            b'0'..=b'9' if !matches!(*current_atom, Element::Number(_)) => {
                Some(IMPLICIT_MULTIPLICATION_INFO)
            },
            _ => None,
        }
    }

    fn assert_eq_consume(&mut self, tok: u8) -> Result<(), ParseError> {
        if !self.consume_if_eq(tok) {
            yeet!(ParseError::new_expected_token(self, tok));
        }
        Ok(())
    }
}

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
#[error("{kind}")]
pub struct ParseError {
    kind: ErrorKind,
    span: miette::SourceSpan,
    src: String,
}

extern crate alloc;
use alloc::fmt;
use std::iter::Iterator;
impl miette::Diagnostic for ParseError {
    #[inline]
    fn help(&self) -> Option<Box<dyn fmt::Display + '_>> {
        let message = match self.kind {
            ErrorKind::UnexpectedEndOfExpression => {
                "Something might be missing here?".to_owned()
            },
            ErrorKind::UnexpectedToken(_) => "Try removing it?".to_owned(),
            ErrorKind::MalformedNumber(_) => {
                "Did you enter a number with multiple decimal points?"
                    .to_owned()
            },
            ErrorKind::IllegalCharacter(_) => {
                "Try removing this character.".to_owned()
            },
            ErrorKind::ExpectedToken(tok) => {
                format!("Try adding a `{tok}` here.")
            },
            ErrorKind::VariableNotDeclared(_, ref availables) => {
                format!(
                    "Try replacing it with one of the following: `{}`.",
                    availables.join("`, `")
                )
            },
            ErrorKind::TooManyArguments(expected, got) => {
                let excess = got - usize::from(expected);
                format!(
                    "Try removing {excess} argument{}.",
                    if excess > 1 { "s" } else { "" }
                )
            },
            ErrorKind::TooFewArguments(expected, got) => {
                let missing = usize::from(expected) - got;
                format!(
                    "Try adding {missing} argument{}.",
                    if missing > 1 { "s" } else { "" }
                )
            },
            ErrorKind::MissingArgument => {
                "Either remove comma or add argument.".to_owned()
            },
        };
        Some(Box::new(message))
    }

    #[inline]
    fn labels(
        &self,
    ) -> Option<Box<dyn Iterator<Item = miette::LabeledSpan> + '_>> {
        Some(Box::new(
            <[_]>::into_vec(Box::new([miette::LabeledSpan::new_with_span(
                Some(fmt::format(format_args!("here"))),
                self.span,
            )]))
            .into_iter(),
        ))
    }

    #[inline]
    fn source_code(&self) -> Option<&dyn miette::SourceCode> {
        Some(&self.src)
    }
}

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
#[non_exhaustive]
pub enum ErrorKind {
    #[error("Unexpected end of expression")]
    UnexpectedEndOfExpression,
    #[error("Unexpected token: `{0}`")]
    UnexpectedToken(char),
    #[error("Malformed number: `{0}`")]
    MalformedNumber(String),
    #[error("Illegal character: `{0}`")]
    IllegalCharacter(char),
    #[error("Expected token: `{0}`")]
    ExpectedToken(char),
    #[error("Variable not previously declared: `{0}`")]
    VariableNotDeclared(String, Vec<String>),
    #[error("Too few arguments for function call, expected {0} got {1}")]
    TooFewArguments(u8, usize),
    #[error("Too many arguments for function call, expected {0} got {1}")]
    TooManyArguments(u8, usize),
    #[error("Missing argument for function call")]
    MissingArgument,
}

impl ParseError {
    #[cold]
    fn new_unexpected_end_of_expression(parser: &ParserImpl) -> Self {
        Self {
            kind: ErrorKind::UnexpectedEndOfExpression,
            // - 1 because we want to point to the last character
            // (without it we would point to a `None` value)
            span: (parser.cursor - 1).into(),
            src: trust_me!(str::from_utf8_unchecked(parser.input)).to_owned(),
        }
    }

    #[cold]
    fn new_unexpected_token(parser: &ParserImpl, tok: u8) -> Self {
        Self {
            kind: ErrorKind::UnexpectedToken(char::from(tok)),
            span: parser.cursor.into(),
            src: trust_me!(str::from_utf8_unchecked(parser.input)).to_owned(),
        }
    }

    #[cold]
    fn new_malformed_number(parser: &ParserImpl, ident: &str) -> Self {
        let num_len = ident.len();
        Self {
            kind: ErrorKind::MalformedNumber(ident.to_owned()),
            span: (parser.cursor - num_len, num_len).into(),
            src: trust_me!(str::from_utf8_unchecked(parser.input)).to_owned(),
        }
    }

    #[cold]
    fn new_illegal_character(parser: &ParserImpl, tok: u8) -> Self {
        Self {
            kind: ErrorKind::IllegalCharacter(char::from(tok)),
            span: parser.cursor.into(),
            src: trust_me!(str::from_utf8_unchecked(parser.input)).to_owned(),
        }
    }

    #[cold]
    fn new_expected_token(parser: &ParserImpl, tok: u8) -> Self {
        Self {
            kind: ErrorKind::ExpectedToken(char::from(tok)),
            span: parser.cursor.into(),
            src: trust_me!(str::from_utf8_unchecked(parser.input)).to_owned(),
        }
    }

    #[cold]
    fn new_variable_not_declared(
        input: &str,
        var: &str,
        availables: Vec<String>,
    ) -> Self {
        Self {
            kind: ErrorKind::VariableNotDeclared(var.to_owned(), availables),
            span: (0, input.len()).into(),
            src: input.to_owned(),
        }
    }

    #[cold]
    fn new_too_few_arguments(
        parser: &ParserImpl,
        expected: u8,
        got: usize,
        start: usize,
    ) -> Self {
        Self {
            kind: ErrorKind::TooFewArguments(expected, got),
            span: (start..parser.cursor).into(),
            src: trust_me!(str::from_utf8_unchecked(parser.input)).to_owned(),
        }
    }

    #[cold]
    fn new_too_many_arguments(
        parser: &ParserImpl,
        expected: u8,
        got: usize,
        start: usize,
    ) -> Self {
        Self {
            kind: ErrorKind::TooManyArguments(expected, got),
            span: (start..parser.cursor).into(),
            src: trust_me!(str::from_utf8_unchecked(parser.input)).to_owned(),
        }
    }

    #[cold]
    fn new_missing_argument(parser: &ParserImpl) -> Self {
        Self {
            kind: ErrorKind::MissingArgument,
            span: (parser.cursor - 1, 2).into(),
            src: trust_me!(str::from_utf8_unchecked(parser.input)).to_owned(),
        }
    }
}
