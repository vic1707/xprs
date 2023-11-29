/* Crate imports */
use super::macros::assert_f64_eq;
use crate::utils::factorial::factorial;

#[test]
fn test_float_factorial() {
    // negatives
    assert!(factorial(-f64::INFINITY).is_nan());
    assert!(factorial(-5.0).is_nan());
    assert!(factorial(-2.5).is_nan());
    assert!(factorial(-1.0).is_nan());
    assert_f64_eq!(factorial(-0.0), 1.0);
    // positives
    assert_f64_eq!(factorial(0.0), 1.0);
    assert_f64_eq!(factorial(1.0), 1.0);
    assert!(factorial(2.5).is_nan());
    assert_f64_eq!(factorial(2.0), 2.0);
    assert_f64_eq!(factorial(5.0), 120.0);
    // approaching the limit
    assert_f64_eq!(factorial(169.0), 4.269_068_009_004_702_7e304);
    assert_f64_eq!(factorial(170.0), 7.257_415_615_307_994e306);
    // at the limit
    assert!(factorial(171.0).is_infinite());
    assert!(factorial(f64::INFINITY).is_infinite());
}
