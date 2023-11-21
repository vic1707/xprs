/* Crate imports */
use crate::token::Operator;

/// Constant representing no precedence.
pub const NO_PRECEDENCE: usize = 0;
/// Constant representing unary operator precedence.
pub const UNOP_PRECEDENCE: usize = 3;

#[cfg(feature = "pejmdas")]
const IMPLICIT_MULTIPLICATION_PRECEDENCE: usize = 3;
#[cfg(feature = "pemdas")]
const IMPLICIT_MULTIPLICATION_PRECEDENCE: usize = get_for_op(&Operator::Times);

/// Constant representing the precedence of implicit multiplication.
/// The actual value depends on the feature configuration.
pub const IMPLICIT_MULTIPLICATION_INFO: (Operator, usize) =
    (Operator::Times, IMPLICIT_MULTIPLICATION_PRECEDENCE);

/// Retrieves the precedence value for a given operator.
///
/// # Parameters
///
/// - `op`: The operator for which to retrieve the precedence.
///
/// # Returns
///
/// The precedence value.
pub const fn get_for_op(op: &Operator) -> usize {
    match *op {
        Operator::Plus | Operator::Minus => 1,
        Operator::Times | Operator::Divide | Operator::Modulo => 2,
        // uses `4` because `pejmdas` feature uses `3`
        Operator::Power => 4,
    }
}
