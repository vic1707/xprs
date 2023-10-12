/* Built-in imports */
use std::collections::HashMap;
/* Crate imports */
use crate::utils::Function;

#[derive(Debug, Default, PartialEq)]
#[non_exhaustive]
pub struct Context<'a> {
    pub vars: HashMap<&'a str, f64>,
    pub funcs: HashMap<&'a str, Function>,
}
