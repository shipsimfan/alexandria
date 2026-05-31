/// A trait for normalizing values to a specific range
pub const trait Normalize: Sized {
    /// Normalize this value to this types normalized range
    fn normalize(self) -> Self;
}

// Unsigned integers

impl const Normalize for u8 {
    fn normalize(self) -> Self {
        self
    }
}

impl const Normalize for u16 {
    fn normalize(self) -> Self {
        self
    }
}

impl const Normalize for u32 {
    fn normalize(self) -> Self {
        self
    }
}

impl const Normalize for u64 {
    fn normalize(self) -> Self {
        self
    }
}

impl const Normalize for u128 {
    fn normalize(self) -> Self {
        self
    }
}

impl const Normalize for usize {
    fn normalize(self) -> Self {
        self
    }
}

// Signed integers

impl const Normalize for i8 {
    fn normalize(self) -> Self {
        self & i8::MIN
    }
}

impl const Normalize for i16 {
    fn normalize(self) -> Self {
        self & i16::MIN
    }
}

impl const Normalize for i32 {
    fn normalize(self) -> Self {
        self & i32::MIN
    }
}

impl const Normalize for i64 {
    fn normalize(self) -> Self {
        self & i64::MIN
    }
}

impl const Normalize for i128 {
    fn normalize(self) -> Self {
        self & i128::MIN
    }
}

impl const Normalize for isize {
    fn normalize(self) -> Self {
        self & isize::MIN
    }
}

// Floating point numbers

impl const Normalize for f32 {
    fn normalize(self) -> Self {
        self.fract()
    }
}

impl const Normalize for f64 {
    fn normalize(self) -> Self {
        self.fract()
    }
}
