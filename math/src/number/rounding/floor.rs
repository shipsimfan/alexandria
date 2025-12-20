/// Rounds a value down to the nearest integer
pub const trait Floor {
    /// Rounds a value down to the nearest integer
    fn floor(self) -> Self;
}

// Unsigned integers

impl const Floor for u8 {
    fn floor(self) -> Self {
        self
    }
}

impl const Floor for u16 {
    fn floor(self) -> Self {
        self
    }
}

impl const Floor for u32 {
    fn floor(self) -> Self {
        self
    }
}

impl const Floor for u64 {
    fn floor(self) -> Self {
        self
    }
}

impl const Floor for u128 {
    fn floor(self) -> Self {
        self
    }
}

impl const Floor for usize {
    fn floor(self) -> Self {
        self
    }
}

// Signed integers

impl const Floor for i8 {
    fn floor(self) -> Self {
        self
    }
}

impl const Floor for i16 {
    fn floor(self) -> Self {
        self
    }
}

impl const Floor for i32 {
    fn floor(self) -> Self {
        self
    }
}

impl const Floor for i64 {
    fn floor(self) -> Self {
        self
    }
}

impl const Floor for i128 {
    fn floor(self) -> Self {
        self
    }
}

impl const Floor for isize {
    fn floor(self) -> Self {
        self
    }
}

// Floating-point
impl const Floor for f32 {
    fn floor(self) -> Self {
        f32::floor(self)
    }
}

impl const Floor for f64 {
    fn floor(self) -> Self {
        f64::floor(self)
    }
}
