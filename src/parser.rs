/* Clippy Config */
#![allow(clippy::std_instead_of_core)]
/* Built-in imports */
use core::str;
/* Crate imports */
use crate::{
    context::Context,
    element::{BinOp, Element, FunctionCall, Simplify, UnOp},
    misc::HashSet,
    token::{Identifier, Operator},
    utils::{
        macros::{trust_me, yeet},
        precedence,
    },
    xprs::Xprs,
};
/* Dependencies imports */
use miette::{Diagnostic, SourceSpan};

#[derive(Debug, Default, PartialEq)]
pub struct Parser<'a> {
    ctx: Context<'a>,
}

impl<'a> Parser<'a> {
    #[inline]
    #[must_use]
    pub const fn new_with_ctx(ctx: Context<'a>) -> Self {
        Self { ctx }
    }

    #[inline]
    #[must_use]
    pub const fn ctx(&self) -> &Context {
        &self.ctx
    }

    #[inline]
    #[must_use]
    pub fn ctx_mut<'b>(&'b mut self) -> &'b mut Context<'a> {
        &mut self.ctx
    }

    #[inline]
    pub fn parse<'b>(&'b self, input: &'b str) -> Result<Xprs<'b>, ParseError> {
        let xprs = ParserImpl::parse(input, &self.ctx)?;

        // Check if no unknown variable was found
        if let Some(unknown_var) = self
            .ctx
            .expected_vars
            .as_ref()
            .and_then(|expected| xprs.vars.difference(expected).next())
        {
            yeet!(ParseError::new_variable_not_declared(
                &ParserImpl::new(input, &self.ctx),
                unknown_var,
            ))
        }

        Ok(xprs)
    }
}

struct ParserImpl<'a> {
    input: &'a [u8],
    cursor: usize,
    ctx: &'a Context<'a>,
}

impl<'a> ParserImpl<'a> {
    #[inline]
    #[must_use]
    const fn new(input: &'a str, ctx: &'a Context) -> Self {
        Self {
            input: input.as_bytes(),
            cursor: 0,
            ctx,
        }
    }

    #[inline]
    pub fn parse(
        input: &'a str,
        ctx: &'a Context,
    ) -> Result<Xprs<'a>, ParseError> {
        let mut parser_impl = Self::new(input, ctx);

        let root = parser_impl.element(precedence::NO_PRECEDENCE)?;

        if let Some(&tok) = parser_impl.next() {
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
    ) -> Result<Element<'a>, ParseError> {
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
                let operand = self.element(precedence::UNOP_PRECEDENCE)?;
                UnOp::new_element(operator, operand)
            },
            /* Parenthesis */
            b'(' => {
                self.cursor += 1;
                let el = self.element(precedence::NO_PRECEDENCE)?;
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
            .vars
            .get(name)
            .map(|&value| Identifier::Constant(value))
            .or_else(|| {
                self.ctx
                    .funcs
                    .get(name)
                    .map(|&func| Identifier::Function(func))
            })
            .unwrap_or_else(|| name.into());

        let el = match ident {
            Identifier::Constant(val) => Element::Number(val),
            Identifier::Variable(var) => Element::Variable(var),
            Identifier::Function(func) if Some(&b'(') == self.next() => {
                self.cursor += 1;
                let args = match func.nb_args {
                    Some(nb) => self.parse_arguments(nb)?,
                    None => self.parse_variadic_arguments()?,
                };
                if self.next() != Some(&b')') {
                    yeet!(ParseError::new_expected_token(self, b')'));
                }
                self.cursor += 1;
                FunctionCall::new_element(func, args)
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

    fn parse_arguments(
        &mut self,
        nb_args: u8,
    ) -> Result<Vec<Element<'a>>, ParseError> {
        let args = (1..=nb_args)
            .map(|idx| {
                let arg = self.element(precedence::NO_PRECEDENCE)?;
                // check for comma if not last argument
                if idx != nb_args {
                    if self.next() == Some(&b',') {
                        self.cursor += 1;
                    } else {
                        yeet!(ParseError::new_not_enough_arguments(
                            self, nb_args, idx
                        ));
                    }
                    if self.next() == Some(&b')') {
                        yeet!(ParseError::new_missing_argument(self));
                    }
                }
                Ok(arg)
            })
            .collect::<Result<Vec<Element<'a>>, ParseError>>();
        if self.next() == Some(&b',') {
            yeet!(ParseError::new_too_many_arguments(
                self,
                nb_args,
                nb_args + 1
            ));
        }
        args
    }

    fn parse_variadic_arguments(
        &mut self,
    ) -> Result<Vec<Element<'a>>, ParseError> {
        let mut args = Vec::new();

        loop {
            let arg = self.element(precedence::NO_PRECEDENCE)?;
            args.push(arg);

            // expect either a comma or a closing parenthesis
            match self.next() {
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
        use precedence::IMPLICIT_MULTIPLICATION_INFO;

        let current_byte = *self.next()?;
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
    #[error("Variable not previously declared: `{0}`")]
    VariableNotDeclared(String),
    #[error("Not enough arguments for function call, expected {0} got {1}")]
    NotEnoughArguments(u8, u8),
    #[error("Too many arguments for function call, expected {0} got {1}")]
    TooManyArguments(u8, u8),
    #[error("Missing argument for function call")]
    MissingArgument,
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

    fn new_variable_not_declared(parser: &ParserImpl, var: &str) -> Self {
        Self {
            kind: ErrorKind::VariableNotDeclared(var.to_owned()),
            span: (0, parser.input.len()).into(),
            src: trust_me!(str::from_utf8_unchecked(parser.input)).to_owned(),
        }
    }

    fn new_not_enough_arguments(
        parser: &ParserImpl,
        expected: u8,
        got: u8,
    ) -> Self {
        Self {
            kind: ErrorKind::NotEnoughArguments(expected, got),
            span: parser.cursor.into(),
            src: trust_me!(str::from_utf8_unchecked(parser.input)).to_owned(),
        }
    }

    fn new_too_many_arguments(
        parser: &ParserImpl,
        expected: u8,
        got: u8,
    ) -> Self {
        Self {
            kind: ErrorKind::TooManyArguments(expected, got),
            span: parser.cursor.into(),
            src: trust_me!(str::from_utf8_unchecked(parser.input)).to_owned(),
        }
    }

    fn new_missing_argument(parser: &ParserImpl) -> Self {
        Self {
            kind: ErrorKind::MissingArgument,
            span: parser.cursor.into(),
            src: trust_me!(str::from_utf8_unchecked(parser.input)).to_owned(),
        }
    }
}
