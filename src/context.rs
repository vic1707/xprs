/* Crate imports */
use crate::{
    misc::{HashMap, HashSet},
    token::Function,
};

#[derive(Debug, Default, PartialEq)]
pub struct Context<'a> {
    vars: HashMap<&'a str, f64>,
    funcs: HashMap<&'a str, Function<'a>>,
    expected_vars: Option<HashSet<&'a str>>,
}

impl<'a> Context<'a> {
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
            funcs: HashMap::new(),
            expected_vars: None,
        }
    }

    #[inline]
    #[must_use]
    pub fn with_expected_vars(
        mut self,
        expected_vars: HashSet<&'a str>,
    ) -> Self {
        self.expected_vars = Some(expected_vars);
        self
    }

    #[inline]
    pub fn add_var<T: Into<f64>>(&mut self, name: &'a str, value: T) {
        self.vars.insert(name, value.into());
    }

    #[inline]
    pub fn add_func(&mut self, name: &'a str, func: Function<'a>) {
        self.funcs.insert(name, func);
    }

    #[inline]
    pub fn set_expected_vars(&mut self, expected_vars: HashSet<&'a str>) {
        self.expected_vars = Some(expected_vars);
    }

    #[inline]
    #[must_use]
    pub fn get_var(&self, name: &'a str) -> Option<&f64> {
        self.vars.get(name)
    }

    #[inline]
    #[must_use]
    pub fn get_func(&self, name: &'a str) -> Option<&Function<'a>> {
        self.funcs.get(name)
    }

    #[inline]
    #[must_use]
    pub const fn get_expected_vars(&self) -> Option<&HashSet<&'a str>> {
        self.expected_vars.as_ref()
    }
}
