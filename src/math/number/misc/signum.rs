/// Returns a number that represents the sign of `self`
pub const trait Signum {
    /// Returns a number that represents the sign of `self`
    fn signum(self) -> Self;
}

// Unsigned integers

impl const Signum for u8 {
    fn signum(self) -> Self {
        1
    }
}

impl const Signum for u16 {
    fn signum(self) -> Self {
        1
    }
}

impl const Signum for u32 {
    fn signum(self) -> Self {
        1
    }
}

impl const Signum for u64 {
    fn signum(self) -> Self {
        1
    }
}

impl const Signum for u128 {
    fn signum(self) -> Self {
        1
    }
}

impl const Signum for usize {
    fn signum(self) -> Self {
        1
    }
}

// Signed integers

impl const Signum for i8 {
    fn signum(self) -> Self {
        i8::signum(self)
    }
}

impl const Signum for i16 {
    fn signum(self) -> Self {
        i16::signum(self)
    }
}

impl const Signum for i32 {
    fn signum(self) -> Self {
        i32::signum(self)
    }
}

impl const Signum for i64 {
    fn signum(self) -> Self {
        i64::signum(self)
    }
}

impl const Signum for i128 {
    fn signum(self) -> Self {
        i128::signum(self)
    }
}

impl const Signum for isize {
    fn signum(self) -> Self {
        isize::signum(self)
    }
}

// Floating-point
impl const Signum for f32 {
    fn signum(self) -> Self {
        f32::signum(self)
    }
}

impl const Signum for f64 {
    fn signum(self) -> Self {
        f64::signum(self)
    }
}
