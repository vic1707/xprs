/* Clippy Config */
#![allow(clippy::std_instead_of_core, reason = "HashSet requires std")]
/* Built-in imports */
#[cfg(feature = "miette")]
use core::{fmt, iter};
use core::{num::NonZeroU8, ops::Range, str};
use std::collections::HashSet;
/* Dependencies imports */
use lexical_parse_float::{
    Error as FloatParseError, FromLexicalWithOptions as _, NumberFormatBuilder,
    Options,
};
/* Crate imports */
#[cfg(feature = "compile-time-optimizations")]
use crate::element::Fold as _;
use crate::{
    context::Context,
    element::{BinOp, Element, FunctionCall, UnOp},
    token::{Identifier, Operator},
    utils::{
        hidden_macros::{trust_me, yeet},
        precedence,
    },
    xprs::Xprs,
};

/// Parses mathematical expressions and returns an expression tree.
///
/// The parser takes a mathematical expression as input and produces an
/// expression tree ([`Xprs`]) as output. It supports various mathematical
/// operations, variables, functions, and constants.
///
/// # Example
///
/// ```
/// use xprs::{Parser, Context};
///
/// let mut context = Context::default();
/// let parser = Parser::new_with_ctx(context);
/// let expression = "2 * (x + 1)";
/// let result = parser.parse(expression);
/// assert!(result.is_ok());
/// ```
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Parser<'ctx> {
    /// The context of the parser.
    ctx: Context<'ctx>,
}

impl<'ctx> Parser<'ctx> {
    /// Creates a new parser with the given context.
    #[inline]
    #[must_use]
    pub const fn new_with_ctx(ctx: Context<'ctx>) -> Self {
        Self { ctx }
    }

    /// Returns a reference to the parser's context.
    #[inline]
    #[must_use]
    pub const fn ctx(&self) -> &Context<'ctx> {
        &self.ctx
    }

    /// Returns a mutable reference to the parser's context.
    #[inline]
    pub const fn ctx_mut(&mut self) -> &mut Context<'ctx> {
        &mut self.ctx
    }

    /// Parses the input mathematical expression,
    /// returns an [`Xprs`] if parsing is successful, or a [`ParseError`].
    ///
    /// # Errors
    ///
    /// See [`ParseError`] for more information about the possible errors.
    ///
    /// # Example
    ///
    /// ```
    /// use xprs::Parser;
    ///
    /// let parser = Parser::default();
    /// let expression = "2 * (x + 1)";
    /// let result = parser.parse(expression);
    /// assert!(result.is_ok());
    /// let invalid_expression = "(x + 1";
    /// let result = parser.parse(invalid_expression);
    /// assert!(result.is_err());
    /// ```
    #[inline]
    pub fn parse<'input>(
        &self,
        input: &'input str,
    ) -> Result<Xprs<'input>, ParseError> {
        let xprs = ParserImpl::parse(input, &self.ctx)?;

        // Check if no unknown variable was found
        if let Some(expected) = self.ctx.get_expected_vars()
            && let Some(unknown_var) = xprs.vars.difference(expected).next()
        {
            yeet!(ParseError::new_variable_not_declared(
                input,
                unknown_var,
                expected
                    .iter()
                    .map(|&str| str.to_owned())
                    .collect::<Vec<_>>()
            ))
        }

        Ok(xprs)
    }
}

/// Internal implementation of the parser.
///
/// This structure holds the state and methods for parsing a mathematical
/// expression.
struct ParserImpl<'input, 'ctx> {
    /// The input expression to be parsed.
    /// As a byte slice for faster parsing.
    input: &'input [u8],
    /// The current cursor position in the input expression.
    cursor: usize,
    /// Whether the last parsed atom ended in a numeric literal.
    last_atom_is_number: bool,
    /// The context of the parser.
    ctx: &'ctx Context<'ctx>,
}

impl<'input, 'ctx> ParserImpl<'input, 'ctx> {
    /// Creates a new parser implementation.
    const fn new(input: &'input str, ctx: &'ctx Context<'ctx>) -> Self {
        Self {
            input: input.as_bytes(),
            cursor: 0,
            last_atom_is_number: false,
            ctx,
        }
    }

    /// Parses the input mathematical.
    fn parse(
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

    /// Parses an element of the mathematical expression.
    /// And checks for binary operators with higher precedence.
    /// If a binary operator with higher precedence is found, it parses the
    /// right-hand side of the expression and creates a new binary operation
    /// with the current element as left-hand side and the parsed element as
    /// right-hand side.
    fn element(
        &mut self,
        precedence: usize,
    ) -> Result<Element<'input>, ParseError> {
        let mut el = self.atom()?;
        #[cfg(feature = "compile-time-optimizations")]
        {
            el = el.fold();
        };

        // Right-associative unary operators
        if self.consume_if_eq(b'!') {
            el = UnOp::new_element(Operator::Factorial, el);
            self.last_atom_is_number = false;
            #[cfg(feature = "compile-time-optimizations")]
            {
                el = el.fold();
            };
        }

        // BinOp with higher precedence
        while let Some((op, op_precedence)) =
            self.get_operator_infos(precedence)
        {
            let rhs = self.element(op_precedence)?;
            #[cfg(feature = "compile-time-optimizations")]
            {
                el = match (el, rhs) {
                    // Literal arithmetic needs no temporary boxed node.
                    (Element::Number(left), Element::Number(right)) => {
                        Element::Number(op.eval_binary(left, right))
                    },
                    (left, right) => BinOp::new_element(op, left, right).fold(),
                };
            }
            #[cfg(not(feature = "compile-time-optimizations"))]
            {
                el = BinOp::new_element(op, el, rhs);
            }
        }

        Ok(el)
    }

    /// Parses an atomic element of the mathematical expression.
    fn atom(&mut self) -> Result<Element<'input>, ParseError> {
        let Some(&next) = self.next_trim() else {
            yeet!(ParseError::new_unexpected_end_of_expression(self));
        };
        let atom = match next {
            /* Number */
            b'0'..=b'9' | b'.' => self.parse_number()?,
            /* Identifier */
            b'A'..=b'Z' | b'a'..=b'z' => self.parse_identifier()?,
            /* Unary expression */
            op @ (b'+' | b'-') => {
                self.cursor += 1;
                #[expect(
                    clippy::unreachable,
                    reason = "this arm only handles the plus and minus bytes"
                )]
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

        // Unary signs retain the operand's ending; groups and calls end with ')'.
        if !matches!(next, b'+' | b'-') {
            self.last_atom_is_number = matches!(next, b'0'..=b'9' | b'.');
        }
        Ok(atom)
    }

    /// Parses an identifier in the mathematical expression.
    fn parse_identifier(&mut self) -> Result<Element<'input>, ParseError> {
        let identifier_start = self.cursor;
        let name = self.take_while(
            |&ch| matches!(ch, b'_' | b'\'' | b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9'),
        );

        // checks for contexts or built-in functions
        // else defaults to variable
        let ident = self
            .ctx
            .get(name)
            .cloned()
            .map_or_else(|| Identifier::from_str(name), Into::into);

        let el = match ident {
            Identifier::Constant(val) => Element::Number(val),
            Identifier::Variable(var) => Element::Variable(var),
            Identifier::Function(func) if self.consume_if_eq(b'(') => {
                let args =
                    self.parse_arguments(func.nb_args.map_or(0, usize::from))?;
                self.assert_eq_consume(b')')?;
                if let Some(nb_args) = func.nb_args {
                    use core::cmp::Ordering::{Equal, Greater, Less};
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

    /// Parses a number in the mathematical expression.
    fn parse_number(&mut self) -> Result<Element<'input>, ParseError> {
        const FORMAT: u128 = NumberFormatBuilder::new()
            .digit_separator(NonZeroU8::new(b'_'))
            .leading_digit_separator(true)
            .trailing_digit_separator(true)
            .internal_digit_separator(true)
            .consecutive_digit_separator(true)
            .build_strict();
        const OPTIONS: Options = Options::new();

        let begin = self.cursor;
        self.skip_while(|&ch| matches!(ch, b'0'..=b'9' | b'.' | b'_'));
        let mantissa_end = self.cursor;
        // An e/E without a following number remains an identifier (Euler's e).
        let numeric_start = |candidate: Option<&u8>| {
            candidate.is_some_and(|&byte| matches!(byte, b'0'..=b'9' | b'.'))
        };
        let might_be_exponent = matches!(self.current(), Some(&b'e' | &b'E'));
        let is_exponent_with_sign = might_be_exponent
            && matches!(self.next(), Some(&b'+' | &b'-'))
            && numeric_start(self.next_at(2));
        let is_exponent = might_be_exponent && numeric_start(self.next());

        if is_exponent_with_sign || is_exponent {
            self.cursor += 1 + usize::from(is_exponent_with_sign);
            self.skip_while(|&ch| matches!(ch, b'0'..=b'9' | b'.' | b'_'));
        }
        let end = self.cursor;
        #[expect(
            clippy::indexing_slicing,
            reason = "both offsets come from scanning the input"
        )]
        let bytes = &self.input[begin..end];
        let (mantissa, tail) = bytes.split_at(mantissa_end - begin);
        let parse = |digits: &[u8]| {
            f64::from_lexical_with_options::<FORMAT>(digits, &OPTIONS)
        };
        let number = if let Some((_, exponent)) = tail.split_first()
            && let Some(dot) = exponent.iter().position(|&byte| byte == b'.')
        {
            parse(exponent).and_then(|power| {
                let (integer, fraction) = exponent.split_at(dot);
                if fraction
                    .iter()
                    .skip(1)
                    .all(|&byte| matches!(byte, b'0' | b'_'))
                {
                    // Integral exponents written with .0 keep correctly rounded conversion.
                    if integer.iter().any(u8::is_ascii_digit) {
                        parse(bytes.split_at(mantissa.len() + 1 + dot).0)
                    } else {
                        parse(mantissa)
                    }
                } else {
                    // Fractional scientific exponents use ordinary f64 power arithmetic.
                    Self::parse_fractional_number::<FORMAT>(
                        mantissa, power, &OPTIONS,
                    )
                }
            })
        } else {
            parse(bytes)
        }
        .map_err(
            #[cold]
            |_err| {
                ParseError::new_malformed_number(
                    self,
                    // The scanner accepts only ASCII number characters.
                    trust_me!(str::from_utf8_unchecked(bytes)),
                )
            },
        )?;

        Ok(Element::Number(number))
    }

    /// Normalizes a fractional scientific literal before applying its power.
    fn parse_fractional_number<const FORMAT: u128>(
        mantissa: &[u8],
        power: f64,
        options: &Options,
    ) -> Result<f64, FloatParseError> {
        // Validate the full spelling even when its standalone value over/underflows.
        let _validated =
            f64::from_lexical_with_options::<FORMAT>(mantissa, options)?;
        // In [1, 10), f64 rounding midpoints have at most 53 fractional decimal
        // digits. Retain those plus a sticky digit for any nonzero discarded tail.
        let mut normalized = [b'0'; 56];
        normalized[1] = b'.';
        let mut used = 0;
        let mut integer = true;
        let mut decimal_shift = -1_i64;
        let mut sticky = false;
        for &byte in mantissa {
            match byte {
                b'.' => integer = false,
                b'_' => {},
                _ => {
                    decimal_shift += i64::from(integer);
                    if used == 0 && byte == b'0' {
                        decimal_shift -= 1;
                    } else if used == 0 {
                        normalized[0] = byte;
                        used = 2;
                    } else if used < 55 {
                        #[expect(
                            clippy::indexing_slicing,
                            reason = "used is bounded by the normalized digit capacity"
                        )]
                        let digit = &mut normalized[used];
                        *digit = byte;
                        used += 1;
                    } else {
                        sticky |= byte != b'0';
                    }
                },
            }
        }
        if used == 0 {
            return Ok(0.0);
        }
        if sticky {
            normalized[55] = b'1';
            used = 56;
        }
        #[expect(
            clippy::indexing_slicing,
            reason = "used is bounded by the normalized digit capacity"
        )]
        let value = f64::from_lexical_with_options::<FORMAT>(
            &normalized[..used],
            options,
        )?;
        #[expect(
            clippy::as_conversions,
            clippy::cast_precision_loss,
            reason = "decimal position is combined with an f64 exponent; loss requires petabyte-sized input"
        )]
        let exponent = power + decimal_shift as f64;
        // Keep the power normal until the final multiplication, so neither an
        // overflowing coefficient nor an underflowing factor destroys the result.
        Ok(if exponent > 308.0 {
            value * 10.0_f64.powf(exponent - 308.0) * 1e308
        } else if exponent < -308.0 {
            value * 10.0_f64.powf(exponent + 308.0) * 1e-308
        } else {
            value * 10.0_f64.powf(exponent)
        })
    }

    /// Parses a list of arguments in a function call.
    fn parse_arguments(
        &mut self,
        capacity: usize,
    ) -> Result<Vec<Element<'input>>, ParseError> {
        let mut args = Vec::with_capacity(capacity);
        if self.next_trim() == Some(&b')') {
            return Ok(args);
        }

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

    /// Parses a single argument in a function call.
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

    /// Takes characters while the given predicate is true and returns the
    /// corresponding substring.
    fn take_while(&mut self, predicate: fn(&u8) -> bool) -> &'input str {
        let start = self.cursor;
        self.skip_while(predicate);
        let end = self.cursor;
        trust_me!(
            #[expect(
                clippy::indexing_slicing,
                reason = "both offsets come from scanning the input"
            )]
            str::from_utf8_unchecked(&self.input[start..end])
        )
    }
}

impl ParserImpl<'_, '_> {
    /// Skips characters while the given predicate is true.
    fn skip_while(&mut self, predicate: fn(&u8) -> bool) {
        while self.current().is_some_and(predicate) {
            self.cursor += 1;
        }
    }

    /// Consumes the next character if it matches the given one.
    fn consume_if_eq(&mut self, tok: u8) -> bool {
        let eq = self.next_trim() == Some(&tok);
        self.cursor += usize::from(eq);
        eq
    }

    /// Returns the current character.
    fn current(&self) -> Option<&u8> {
        self.input.get(self.cursor)
    }

    /// Returns the current character after trimming whitespaces.
    fn next_trim(&mut self) -> Option<&u8> {
        self.skip_while(u8::is_ascii_whitespace);
        self.current()
    }

    /// Returns the next character.
    fn next(&self) -> Option<&u8> {
        self.input.get(self.cursor + 1)
    }

    /// Returns the character at the given offset.
    fn next_at(&self, offset: usize) -> Option<&u8> {
        self.input.get(self.cursor + offset)
    }

    /// Returns the operator and its precedence if it's a valid operator.
    fn get_operator_infos(
        &mut self,
        precedence: usize,
    ) -> Option<(Operator, usize)> {
        use precedence::IMPLICIT_MULTIPLICATION_INFO;

        let current_byte = *self.next_trim()?;
        // check for binary operator
        if let Ok(op) = Operator::try_from(current_byte)
            && op != Operator::Factorial
        {
            let op_p = precedence::get_for_op(op);
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
            b'A'..=b'Z' | b'a'..=b'z' | b'(' => {
                Some(IMPLICIT_MULTIPLICATION_INFO)
            },
            // Consecutive numeric literals need an explicit operator, even
            // when the preceding literal is inside a larger binary expression.
            b'0'..=b'9' if !self.last_atom_is_number => {
                Some(IMPLICIT_MULTIPLICATION_INFO)
            },
            _ => None,
        }
    }

    /// Asserts that the next character is equal to the given one and consumes
    /// it if it is.
    fn assert_eq_consume(&mut self, tok: u8) -> Result<(), ParseError> {
        if !self.consume_if_eq(tok) {
            yeet!(ParseError::new_expected_token(self, tok));
        }
        Ok(())
    }
}

/// Represents an error that occurred during parsing.
#[derive(Debug, Eq, PartialEq, thiserror::Error, Clone, Hash)]
#[error("{kind}")]
pub struct ParseError {
    /// The kind of parsing error.
    kind: ErrorKind,
    /// The span of the error in the source input.
    span: Range<usize>,
    /// The source input string.
    src: String,
}

#[cfg(feature = "miette")]
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
            ErrorKind::VariableNotDeclared(_, ref available_vars) => {
                format!(
                    "Try replacing it with one of the following: `{}`.",
                    available_vars.join("`, `")
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
        Some(Box::new(iter::once(miette::LabeledSpan::new_with_span(
            Some("here".to_owned()),
            self.span.clone(),
        ))))
    }

    #[inline]
    fn source_code(&self) -> Option<&dyn miette::SourceCode> {
        Some(&self.src)
    }
}

/// Represents the kind of error that occurred during parsing.
#[derive(
    Debug, Eq, PartialEq, thiserror::Error, PartialOrd, Ord, Clone, Hash,
)]
#[non_exhaustive]
pub enum ErrorKind {
    /// Unexpected end of expression error.
    #[error("Unexpected end of expression")]
    UnexpectedEndOfExpression,
    /// Unexpected token error with a specified character.
    #[error("Unexpected token: `{0}`")]
    UnexpectedToken(char),
    /// Malformed number error with a specified string.
    #[error("Malformed number: `{0}`")]
    MalformedNumber(String),
    /// Illegal character error with a specified character.
    #[error("Illegal character: `{0}`")]
    IllegalCharacter(char),
    /// Expected token error with a specified character.
    #[error("Expected token: `{0}`")]
    ExpectedToken(char),
    /// Variable not previously declared error with variable name and available suggestions.
    #[error("Variable not previously declared: `{0}`")]
    VariableNotDeclared(String, Vec<String>),
    /// Too few arguments for function call error with expected and actual argument counts.
    #[error("Too few arguments for function call, expected {0} got {1}")]
    TooFewArguments(u8, usize),
    /// Too many arguments for function call error with expected and actual argument counts.
    #[error("Too many arguments for function call, expected {0} got {1}")]
    TooManyArguments(u8, usize),
    /// Missing argument for function call error.
    #[error("Missing argument for function call")]
    MissingArgument,
}

impl ParseError {
    /// Returns the kind of parsing error.
    #[inline]
    #[must_use]
    pub const fn kind(&self) -> &ErrorKind {
        &self.kind
    }

    /// Returns the error's byte range in the original input.
    ///
    /// An empty range points to a position rather than a token.
    #[inline]
    #[must_use]
    pub fn span(&self) -> Range<usize> {
        self.span.clone()
    }

    /// Returns the original input that failed to parse.
    #[inline]
    #[must_use]
    pub const fn input(&self) -> &str {
        self.src.as_str()
    }

    /// Creates a new [`ParseError`] for an unexpected end of expression.
    #[cold]
    fn new_unexpected_end_of_expression(parser: &ParserImpl) -> Self {
        Self {
            kind: ErrorKind::UnexpectedEndOfExpression,
            // - 1 because we want to point to the last character
            // (without it, we would point to a `None` value)
            span: parser.cursor.saturating_sub(1)
                ..parser.cursor.saturating_sub(1),
            src: trust_me!(str::from_utf8_unchecked(parser.input)).to_owned(),
        }
    }

    /// Creates a new [`ParseError`] for an unexpected token.
    #[cold]
    fn new_unexpected_token(parser: &ParserImpl, tok: u8) -> Self {
        Self {
            kind: ErrorKind::UnexpectedToken(char::from(tok)),
            span: parser.cursor..parser.cursor,
            src: trust_me!(str::from_utf8_unchecked(parser.input)).to_owned(),
        }
    }

    /// Creates a new [`ParseError`] for a malformed number.
    #[cold]
    fn new_malformed_number(parser: &ParserImpl, ident: &str) -> Self {
        let num_len = ident.len();
        Self {
            kind: ErrorKind::MalformedNumber(ident.to_owned()),
            span: parser.cursor - num_len..parser.cursor,
            src: trust_me!(str::from_utf8_unchecked(parser.input)).to_owned(),
        }
    }

    /// Creates a new [`ParseError`] for an illegal character.
    #[cold]
    fn new_illegal_character(parser: &ParserImpl, tok: u8) -> Self {
        Self {
            kind: ErrorKind::IllegalCharacter(char::from(tok)),
            span: parser.cursor..parser.cursor,
            src: trust_me!(str::from_utf8_unchecked(parser.input)).to_owned(),
        }
    }

    /// Creates a new [`ParseError`] for an expected token error.
    #[cold]
    fn new_expected_token(parser: &ParserImpl, tok: u8) -> Self {
        Self {
            kind: ErrorKind::ExpectedToken(char::from(tok)),
            span: parser.cursor..parser.cursor,
            src: trust_me!(str::from_utf8_unchecked(parser.input)).to_owned(),
        }
    }

    /// Creates a new [`ParseError`] for a variable not declared error.
    #[cold]
    fn new_variable_not_declared(
        input: &str,
        var: &str,
        available_vars: Vec<String>,
    ) -> Self {
        Self {
            kind: ErrorKind::VariableNotDeclared(
                var.to_owned(),
                available_vars,
            ),
            span: 0..input.len(),
            src: input.to_owned(),
        }
    }

    /// Creates a new [`ParseError`] for too few arguments error.
    #[cold]
    fn new_too_few_arguments(
        parser: &ParserImpl,
        expected: u8,
        got: usize,
        start: usize,
    ) -> Self {
        Self {
            kind: ErrorKind::TooFewArguments(expected, got),
            span: start..parser.cursor,
            src: trust_me!(str::from_utf8_unchecked(parser.input)).to_owned(),
        }
    }

    /// Creates a new [`ParseError`] for too many arguments error.
    #[cold]
    fn new_too_many_arguments(
        parser: &ParserImpl,
        expected: u8,
        got: usize,
        start: usize,
    ) -> Self {
        Self {
            kind: ErrorKind::TooManyArguments(expected, got),
            span: start..parser.cursor,
            src: trust_me!(str::from_utf8_unchecked(parser.input)).to_owned(),
        }
    }

    /// Creates a new [`ParseError`] for a missing argument error.
    #[cold]
    fn new_missing_argument(parser: &ParserImpl) -> Self {
        Self {
            kind: ErrorKind::MissingArgument,
            span: parser.cursor - 1..parser.cursor + 1,
            src: trust_me!(str::from_utf8_unchecked(parser.input)).to_owned(),
        }
    }
}
