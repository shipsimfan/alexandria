/// Returns the integer part of self. This means that non-integer numbers are always truncated
/// towards zero.
pub const trait Trunc {
    /// Returns the integer part of self. This means that non-integer numbers are always truncated
    /// towards zero.
    fn trunc(self) -> Self;
}

impl const Trunc for f32 {
    fn trunc(self) -> Self {
        f32::trunc(self)
    }
}

impl const Trunc for f64 {
    fn trunc(self) -> Self {
        f64::trunc(self)
    }
}
