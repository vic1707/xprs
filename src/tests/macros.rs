macro_rules! assert_f64_eq {
    ($left:expr, $right:expr) => {
        assert!(($left - $right).abs() < f64::EPSILON);
    };
    ($left:expr, $right:expr, $($arg:tt)+) => {
        assert!(($left - $right).abs() < f64::EPSILON, $($arg)+);
    };
}

pub(crate) use assert_f64_eq;
