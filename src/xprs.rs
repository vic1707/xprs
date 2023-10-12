/* Built-in imports */
use core::fmt;
/* Crate imports */
use crate::element::Element;

#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub struct Xprs<'a> {
    pub root: Element<'a>,
}

impl fmt::Display for Xprs<'_> {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "{}", self.root)
    }
}
