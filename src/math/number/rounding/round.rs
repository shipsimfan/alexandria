/// Rounds a value to the nearest integer
pub const trait Round {
    /// Rounds a value to the nearest integer
    fn round(self) -> Self;
}

// Unsigned integers

impl const Round for u8 {
    fn round(self) -> Self {
        self
    }
}

impl const Round for u16 {
    fn round(self) -> Self {
        self
    }
}

impl const Round for u32 {
    fn round(self) -> Self {
        self
    }
}

impl const Round for u64 {
    fn round(self) -> Self {
        self
    }
}

impl const Round for u128 {
    fn round(self) -> Self {
        self
    }
}

impl const Round for usize {
    fn round(self) -> Self {
        self
    }
}

// Signed integers

impl const Round for i8 {
    fn round(self) -> Self {
        self
    }
}

impl const Round for i16 {
    fn round(self) -> Self {
        self
    }
}

impl const Round for i32 {
    fn round(self) -> Self {
        self
    }
}

impl const Round for i64 {
    fn round(self) -> Self {
        self
    }
}

impl const Round for i128 {
    fn round(self) -> Self {
        self
    }
}

impl const Round for isize {
    fn round(self) -> Self {
        self
    }
}

// Floating-point
impl const Round for f32 {
    fn round(self) -> Self {
        f32::round(self)
    }
}

impl const Round for f64 {
    fn round(self) -> Self {
        f64::round(self)
    }
}
