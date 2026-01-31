/// A value which can be checked for approximate equality
pub const trait ApproxEq: Sized {
    /// The type to use for `epsilon`
    type Epsilon = Self;

    /// Is this value approximately equal to `other` (with-in `epsilon`)
    fn approx_eq(self, other: Self, epsilon: Self::Epsilon) -> bool;
}

// Unsigned integers

impl const ApproxEq for u8 {
    fn approx_eq(self, other: Self, epsilon: Self) -> bool {
        self.abs_diff(other) <= epsilon
    }
}

impl const ApproxEq for u16 {
    fn approx_eq(self, other: Self, epsilon: Self) -> bool {
        self.abs_diff(other) <= epsilon
    }
}

impl const ApproxEq for u32 {
    fn approx_eq(self, other: Self, epsilon: Self) -> bool {
        self.abs_diff(other) <= epsilon
    }
}

impl const ApproxEq for u64 {
    fn approx_eq(self, other: Self, epsilon: Self) -> bool {
        self.abs_diff(other) <= epsilon
    }
}

impl const ApproxEq for u128 {
    fn approx_eq(self, other: Self, epsilon: Self) -> bool {
        self.abs_diff(other) <= epsilon
    }
}

impl const ApproxEq for usize {
    fn approx_eq(self, other: Self, epsilon: Self) -> bool {
        self.abs_diff(other) <= epsilon
    }
}

// Signed integers

impl const ApproxEq for i8 {
    fn approx_eq(self, other: Self, epsilon: Self) -> bool {
        if epsilon < 0 {
            return false;
        }

        self.abs_diff(other) <= epsilon as _
    }
}

impl const ApproxEq for i16 {
    fn approx_eq(self, other: Self, epsilon: Self) -> bool {
        if epsilon < 0 {
            return false;
        }

        self.abs_diff(other) <= epsilon as _
    }
}

impl const ApproxEq for i32 {
    fn approx_eq(self, other: Self, epsilon: Self) -> bool {
        if epsilon < 0 {
            return false;
        }

        self.abs_diff(other) <= epsilon as _
    }
}

impl const ApproxEq for i64 {
    fn approx_eq(self, other: Self, epsilon: Self) -> bool {
        if epsilon < 0 {
            return false;
        }

        self.abs_diff(other) <= epsilon as _
    }
}

impl const ApproxEq for i128 {
    fn approx_eq(self, other: Self, epsilon: Self) -> bool {
        if epsilon < 0 {
            return false;
        }

        self.abs_diff(other) <= epsilon as _
    }
}

impl const ApproxEq for isize {
    fn approx_eq(self, other: Self, epsilon: Self) -> bool {
        if epsilon < 0 {
            return false;
        }

        self.abs_diff(other) <= epsilon as _
    }
}

// Floating-point
impl const ApproxEq for f32 {
    fn approx_eq(self, other: Self, epsilon: Self) -> bool {
        (self - other).abs() <= epsilon
    }
}

impl const ApproxEq for f64 {
    fn approx_eq(self, other: Self, epsilon: Self) -> bool {
        (self - other).abs() <= epsilon
    }
}
