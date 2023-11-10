/* Crate imports */
use crate::{
    misc::{HashMap, HashSet},
    token::Function,
};

#[derive(Debug, Default, PartialEq)]
pub struct Context<'names> {
    vars: HashMap<&'names str, f64>,
    funcs: HashMap<&'names str, Function>,
    expected_vars: Option<HashSet<&'names str>>,
}

impl<'names> Context<'names> {
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
        expected_vars: HashSet<&'names str>,
    ) -> Self {
        self.expected_vars = Some(expected_vars);
        self
    }

    #[inline]
    pub fn add_var<T: Into<f64>>(&mut self, name: &'names str, value: T) {
        self.vars.insert(name, value.into());
    }

    #[inline]
    pub fn add_func(&mut self, name: &'names str, func: Function) {
        self.funcs.insert(name, func);
    }

    #[inline]
    pub fn set_expected_vars(&mut self, expected_vars: HashSet<&'names str>) {
        self.expected_vars = Some(expected_vars);
    }

    #[inline]
    #[must_use]
    pub fn get_var(&self, name: &str) -> Option<&f64> {
        self.vars.get(name)
    }

    #[inline]
    #[must_use]
    pub fn get_func(&self, name: &str) -> Option<&Function> {
        self.funcs.get(name)
    }

    #[inline]
    #[must_use]
    pub const fn get_expected_vars(&self) -> Option<&HashSet<&str>> {
        self.expected_vars.as_ref()
    }
}
