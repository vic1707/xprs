/* Crate imports */
use crate::token::Operator;

pub const NO_PRECEDENCE: usize = 0;
pub const UNOP_PRECEDENCE: usize = 3;
pub const FN_PRECEDENCE: usize = 5;

#[cfg(feature = "pejmdas")]
pub const IMPLICIT_MULTIPLICATION_PRECEDENCE: usize = 3;
#[cfg(feature = "pemdas")]
pub const IMPLICIT_MULTIPLICATION_PRECEDENCE: usize =
    get_for_op(&Operator::Times);

pub const IMPLICIT_MULTIPLICATION_INFO: (Operator, usize) =
    (Operator::Times, IMPLICIT_MULTIPLICATION_PRECEDENCE);

pub const fn get_for_op(op: &Operator) -> usize {
    match *op {
        Operator::Plus | Operator::Minus => 1,
        Operator::Times | Operator::Divide | Operator::Modulo => 2,
        // uses `4` because `pejmdas` feature uses `3`
        Operator::Power => 4,
    }
}
