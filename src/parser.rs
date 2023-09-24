/* Built-in imports */
use core::str;
/* Crate imports */
use crate::{
    element::{BinOp, Element, FunctionCall, UnOp},
    macros::{trust_me, yeet},
    token::{Identifier, Operator},
};
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
    pub fn parse(&mut self) -> Result<Element<'a>, &'static str> {
        let root = self.element(NO_PERCEDENCE)?;
        if let Some(tok) = self.next() {
            println!("[END] Unexpected token: {:?}", char::from(*tok));
            yeet!("Expected EOF");
        }
        Ok(root)
    }

    fn element(
        &mut self,
        precedence: usize,
    ) -> Result<Element<'a>, &'static str> {
        let mut el = self.atom()?;

        while let Some(op) = self.get_operator(&el, precedence) {
            let current_precedence = BinOp::precedence(&op);
            let rhs = self.element(current_precedence)?;
            el = Element::BinOp(Box::new(BinOp::new(op, el, rhs)));
        }

        Ok(el)
    }

    fn atom(&mut self) -> Result<Element<'a>, &'static str> {
        let atom = match *(self.next().ok_or("Unexpected EOF")?) {
            /* Number */
            b'0'..=b'9' | b'.' => Element::Number(self.parse_number()?),
            /* Unary expression */
            op @ (b'+' | b'-') => {
                self.cursor += 1;
                let operator = Operator::try_from(op)?;
                let operand = self.element(UnOp::PRECEDENCE)?;
                Element::UnOp(Box::new(UnOp::new(operator, operand)))
            },
            /* Parenthesis */
            b'(' => {
                self.cursor += 1;
                let el = self.element(NO_PERCEDENCE)?;
                if self.next() != Some(&b')') {
                    yeet!("Expected ')'");
                }
                self.cursor += 1;
                el
            },
            /* Identifier */
            b'a'..=b'z' => match self.parse_identifier()? {
                Identifier::Constant(val) => Element::Number(val),
                Identifier::Variable(var) => Element::Variable(var),
                Identifier::Function(func) if Some(&b'(') == self.next() => {
                    let el = self.element(FunctionCall::PRECEDENCE)?;
                    Element::Function(Box::new(FunctionCall::new(func, el)))
                },
                Identifier::Function(_) => yeet!("Expected '('"),
            },
            tok => {
                println!("Unexpected token: {:?}", char::from(tok));
                yeet!("Unexpected Token")
            },
        };

        Ok(atom)
    }

    fn parse_identifier(&mut self) -> Result<Identifier<'a>, &'static str> {
        let start = self.cursor;
        self.skip_while(u8::is_ascii_lowercase);
        let end = self.cursor;

        let ident = trust_me!(str::from_utf8_unchecked(
            self.input.get(start..end).ok_or("Unreachable")?
        ));

        Ok(ident.into())
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

    fn parse_number(&mut self) -> Result<f64, &'static str> {
        let start = self.cursor;
        self.skip_while(|&ch| matches!(ch, b'0'..=b'9' | b'.' | b'e' | b'E'));
        let end = self.cursor;

        let ident = self.input.get(start..end).ok_or("Unreachable")?;

        let num = trust_me!(str::from_utf8_unchecked(ident))
            .parse()
            .map_err(|_err| "Failed to parse number")?;

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
