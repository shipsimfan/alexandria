/// Returns `true` if this value is NaN
pub const trait IsNaN {
    /// Returns `true` if this value is NaN
    fn is_nan(self) -> bool;
}

// Unsigned integers

impl const IsNaN for u8 {
    fn is_nan(self) -> bool {
        false
    }
}

impl const IsNaN for u16 {
    fn is_nan(self) -> bool {
        false
    }
}

impl const IsNaN for u32 {
    fn is_nan(self) -> bool {
        false
    }
}

impl const IsNaN for u64 {
    fn is_nan(self) -> bool {
        false
    }
}

impl const IsNaN for u128 {
    fn is_nan(self) -> bool {
        false
    }
}

impl const IsNaN for usize {
    fn is_nan(self) -> bool {
        false
    }
}

// Signed integers

impl const IsNaN for i8 {
    fn is_nan(self) -> bool {
        false
    }
}

impl const IsNaN for i16 {
    fn is_nan(self) -> bool {
        false
    }
}

impl const IsNaN for i32 {
    fn is_nan(self) -> bool {
        false
    }
}

impl const IsNaN for i64 {
    fn is_nan(self) -> bool {
        false
    }
}

impl const IsNaN for i128 {
    fn is_nan(self) -> bool {
        false
    }
}

impl const IsNaN for isize {
    fn is_nan(self) -> bool {
        false
    }
}

// Floating-point
impl const IsNaN for f32 {
    fn is_nan(self) -> bool {
        f32::is_nan(self)
    }
}

impl const IsNaN for f64 {
    fn is_nan(self) -> bool {
        f64::is_nan(self)
    }
}
