/// Returns `true` if this value is positive infinity or negative infinity, and `false` otherwise
pub const trait IsInfinite {
    /// Returns `true` if this value is positive infinity or negative infinity, and `false`
    /// otherwise.
    fn is_infinite(&self) -> bool;
}

// Unsigned integers

impl const IsInfinite for u8 {
    fn is_infinite(&self) -> bool {
        false
    }
}

impl const IsInfinite for u16 {
    fn is_infinite(&self) -> bool {
        false
    }
}

impl const IsInfinite for u32 {
    fn is_infinite(&self) -> bool {
        false
    }
}

impl const IsInfinite for u64 {
    fn is_infinite(&self) -> bool {
        false
    }
}

impl const IsInfinite for u128 {
    fn is_infinite(&self) -> bool {
        false
    }
}

impl const IsInfinite for usize {
    fn is_infinite(&self) -> bool {
        false
    }
}

// Signed integers

impl const IsInfinite for i8 {
    fn is_infinite(&self) -> bool {
        false
    }
}

impl const IsInfinite for i16 {
    fn is_infinite(&self) -> bool {
        false
    }
}

impl const IsInfinite for i32 {
    fn is_infinite(&self) -> bool {
        false
    }
}

impl const IsInfinite for i64 {
    fn is_infinite(&self) -> bool {
        false
    }
}

impl const IsInfinite for i128 {
    fn is_infinite(&self) -> bool {
        false
    }
}

impl const IsInfinite for isize {
    fn is_infinite(&self) -> bool {
        false
    }
}

// Floating-point
impl const IsInfinite for f32 {
    fn is_infinite(&self) -> bool {
        f32::is_infinite(*self)
    }
}

impl const IsInfinite for f64 {
    fn is_infinite(&self) -> bool {
        f64::is_infinite(*self)
    }
}
