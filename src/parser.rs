/* Built-in imports */
use core::iter::Peekable;
/* Crate imports */
use crate::element::{BinOp, FunctionCall, UnOp};
use crate::token::{Identifier, Operator};
use crate::yeet::yeet;
use crate::{Element, Lexer, Token};
/* Constants */
pub const NO_PERCEDENCE: usize = 0;

pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
}

impl<'a> Parser<'a> {
    #[inline]
    #[must_use]
    pub fn new(lexer: Lexer<'a>) -> Self {
        Parser {
            lexer: lexer.peekable(),
        }
    }

    #[inline]
    pub fn parse(&mut self) -> Result<Element<'a>, &'static str> {
        let root = self.element(NO_PERCEDENCE)?;
        if let Some(tok) = self.lexer.next() {
            println!("[END] Unexpected token: {tok:?}");
            yeet!("Expected EOF");
        }
        Ok(root)
    }

    fn element(
        &mut self,
        precedence: usize,
    ) -> Result<Element<'a>, &'static str> {
        let curr = self.lexer.next().ok_or("Unexpected EOF")??;
        let mut el = self.atom(curr)?;

        #[allow(clippy::ref_patterns)]
        while let Some(Ok(Token::Operator(op))) =
            self.lexer.next_if(|tok|
                matches!(tok, &Ok(Token::Operator(ref op)) if BinOp::precedence(op) > precedence)
            )
        {
            let current_precedence = BinOp::precedence(&op);
            if current_precedence <= precedence {
                break;
            }
            let rhs = self.element(current_precedence)?;
            el = Element::BinOp(Box::new(BinOp::new(op, el, rhs)));
        }
        Ok(el)
    }

    fn atom(&mut self, token: Token<'a>) -> Result<Element<'a>, &'static str> {
        let atom = match token {
            Token::Identifier(Identifier::Constant(val)) => {
                Element::Number(val)
            },
            Token::Identifier(Identifier::Variable(var)) => {
                Element::Variable(var)
            },
            Token::Number(val) => Element::Number(val),
            /* Parenthesis */
            Token::LParen => {
                let el = self.element(NO_PERCEDENCE)?;
                if self.lexer.next() != Some(Ok(Token::RParen)) {
                    yeet!("Expected ')'");
                }
                el
            },
            /* Unary */
            Token::Operator(op @ (Operator::Minus | Operator::Plus)) => {
                Element::UnOp(Box::new(UnOp::new(
                    op,
                    self.element(UnOp::PRECEDENCE)?,
                )))
            },
            /* Function */
            Token::Identifier(Identifier::Function(func))
                if self.lexer.peek() == Some(&Ok(Token::LParen)) =>
            {
                let el = self.element(FunctionCall::PRECEDENCE)?;
                Element::Function(Box::new(FunctionCall::new(func, el)))
            },
            /* Errors */
            Token::Identifier(Identifier::Function(_)) => yeet!("Expected '('"),
            tok @ (Token::Operator(_) | Token::RParen) => {
                println!("Unexpected token: {tok:?}");
                yeet!("Unexpected Token")
            },
        };

        Ok(atom)
    }
}
