/// A value which can be checked for approximate equality
pub const trait ApproxEq: Sized {
    /// The type to use for `epsilon`
    type Epsilon = Self;

    /// Is this value approximately equal to `other` (with-in `epsilon`)
    fn approx_eq(self, other: Self, epsilon: Self::Epsilon) -> bool;
}

// Unsigned integers

const impl ApproxEq for u8 {
    fn approx_eq(self, other: Self, epsilon: Self) -> bool {
        self.abs_diff(other) <= epsilon
    }
}

const impl ApproxEq for u16 {
    fn approx_eq(self, other: Self, epsilon: Self) -> bool {
        self.abs_diff(other) <= epsilon
    }
}

const impl ApproxEq for u32 {
    fn approx_eq(self, other: Self, epsilon: Self) -> bool {
        self.abs_diff(other) <= epsilon
    }
}

const impl ApproxEq for u64 {
    fn approx_eq(self, other: Self, epsilon: Self) -> bool {
        self.abs_diff(other) <= epsilon
    }
}

const impl ApproxEq for u128 {
    fn approx_eq(self, other: Self, epsilon: Self) -> bool {
        self.abs_diff(other) <= epsilon
    }
}

const impl ApproxEq for usize {
    fn approx_eq(self, other: Self, epsilon: Self) -> bool {
        self.abs_diff(other) <= epsilon
    }
}

// Signed integers

const impl ApproxEq for i8 {
    fn approx_eq(self, other: Self, epsilon: Self) -> bool {
        if epsilon < 0 {
            return false;
        }

        self.abs_diff(other) <= epsilon as _
    }
}

const impl ApproxEq for i16 {
    fn approx_eq(self, other: Self, epsilon: Self) -> bool {
        if epsilon < 0 {
            return false;
        }

        self.abs_diff(other) <= epsilon as _
    }
}

const impl ApproxEq for i32 {
    fn approx_eq(self, other: Self, epsilon: Self) -> bool {
        if epsilon < 0 {
            return false;
        }

        self.abs_diff(other) <= epsilon as _
    }
}

const impl ApproxEq for i64 {
    fn approx_eq(self, other: Self, epsilon: Self) -> bool {
        if epsilon < 0 {
            return false;
        }

        self.abs_diff(other) <= epsilon as _
    }
}

const impl ApproxEq for i128 {
    fn approx_eq(self, other: Self, epsilon: Self) -> bool {
        if epsilon < 0 {
            return false;
        }

        self.abs_diff(other) <= epsilon as _
    }
}

const impl ApproxEq for isize {
    fn approx_eq(self, other: Self, epsilon: Self) -> bool {
        if epsilon < 0 {
            return false;
        }

        self.abs_diff(other) <= epsilon as _
    }
}

// Floating-point
const impl ApproxEq for f32 {
    fn approx_eq(self, other: Self, epsilon: Self) -> bool {
        (self - other).abs() <= epsilon
    }
}

const impl ApproxEq for f64 {
    fn approx_eq(self, other: Self, epsilon: Self) -> bool {
        (self - other).abs() <= epsilon
    }
}
