/* Crate imports */
use crate::misc::{Function, HashMap, HashSet};

#[derive(Debug, Default, PartialEq)]
#[non_exhaustive]
pub struct Context<'a> {
    pub vars: HashMap<&'a str, f64>,
    pub funcs: HashMap<&'a str, Function>,
    pub expected_vars: Option<HashSet<&'a str>>,
}
