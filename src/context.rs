/* Built-in imports */
use std::collections::{HashMap, HashSet};
/* Crate imports */
use crate::token::Function;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum Symbol {
    Variable(f64),
    Function(Function),
}

impl From<f64> for Symbol {
    #[inline]
    fn from(value: f64) -> Self {
        Self::Variable(value)
    }
}

impl From<Function> for Symbol {
    #[inline]
    fn from(value: Function) -> Self {
        Self::Function(value)
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct Context<'names> {
    symbols: HashMap<&'names str, Symbol>,
    expected_vars: Option<HashSet<&'names str>>,
}

impl<'names> Context<'names> {
    #[inline]
    #[must_use]
    pub fn with_expected_vars(
        mut self,
        expected_vars: HashSet<&'names str>,
    ) -> Self {
        self.expected_vars = Some(expected_vars);
        self
    }

    #[inline]
    #[must_use]
    pub fn with_symbols(mut self, symbols: HashMap<&'names str, Symbol>) -> Self {
        self.symbols = symbols;
        self
    }

    #[inline]
    pub fn add_var<T: Into<f64>>(&mut self, name: &'names str, value: T) {
        self.symbols.insert(name, value.into().into());
    }

    #[inline]
    pub fn add_func(&mut self, name: &'names str, func: Function) {
        self.symbols.insert(name, func.into());
    }

    #[inline]
    pub fn set_expected_vars(&mut self, expected_vars: HashSet<&'names str>) {
        self.expected_vars = Some(expected_vars);
    }

    #[inline]
    #[must_use]
    pub fn get(&self, name: &str) -> Option<&Symbol> {
        self.symbols.get(name)
    }

    #[inline]
    #[must_use]
    pub const fn get_expected_vars(&self) -> Option<&HashSet<&str>> {
        self.expected_vars.as_ref()
    }
}
