/* Modules */
/// The function module.
mod function;
/// The identifier module.
mod identifier;
/// The operator module.
mod operator;
/* Exports */
pub use function::Function;
pub use identifier::Identifier;
pub use operator::Operator;

#[cfg(test)]
pub use function::FnPointer;
