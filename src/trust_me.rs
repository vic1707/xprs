#[doc(hidden)]
macro_rules! trust_me {
  ($($t:tt)*) => {
    unsafe { $($t)* }
  }
}

pub(super) use trust_me;
