/* Crate imports */
use crate::{
    misc::{HashMap, HashSet},
    token::Function,
};

#[derive(Debug, Default, PartialEq)]
#[non_exhaustive]
pub struct Context<'a> {
    pub vars: HashMap<&'a str, f64>,
    pub funcs: HashMap<&'a str, Function<'a>>,
    pub expected_vars: Option<HashSet<&'a str>>,
}
