use word::Word;

/// Clear least significant set bit of `x`.
///
/// Returns 0 if `x` is 0.
///
/// # Intrinsics:
/// - BMI 1.0: blsr.
///
/// # Examples
///
/// ```
/// use bitwise::word::*;
///
/// let n = 0b0110;
/// let s = 0b0100;
///
/// assert_eq!(n.clear_least_significant_one(), s);
/// assert_eq!(clear_least_significant_one(n), s);
/// ```
pub fn clear_least_significant_one<T: Word>(x: T) -> T {
    x & (x - T::one())
}

/// Method version of [`clear_least_significant_one`](fn.clear_least_significant_one.html).
pub trait ClearLeastSignificantOne {
    fn clear_least_significant_one(self) -> Self;
}

impl<T: Word> ClearLeastSignificantOne for T {
    fn clear_least_significant_one(self) -> Self {
        clear_least_significant_one(self)
    }
}
