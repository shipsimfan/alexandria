/// Rounds a value up to the nearest integer
pub const trait Ceil {
    /// Rounds a value up to the nearest integer
    fn ceil(self) -> Self;
}

// Unsigned integers

impl const Ceil for u8 {
    fn ceil(self) -> Self {
        self
    }
}

impl const Ceil for u16 {
    fn ceil(self) -> Self {
        self
    }
}

impl const Ceil for u32 {
    fn ceil(self) -> Self {
        self
    }
}

impl const Ceil for u64 {
    fn ceil(self) -> Self {
        self
    }
}

impl const Ceil for u128 {
    fn ceil(self) -> Self {
        self
    }
}

impl const Ceil for usize {
    fn ceil(self) -> Self {
        self
    }
}

// Signed integers

impl const Ceil for i8 {
    fn ceil(self) -> Self {
        self
    }
}

impl const Ceil for i16 {
    fn ceil(self) -> Self {
        self
    }
}

impl const Ceil for i32 {
    fn ceil(self) -> Self {
        self
    }
}

impl const Ceil for i64 {
    fn ceil(self) -> Self {
        self
    }
}

impl const Ceil for i128 {
    fn ceil(self) -> Self {
        self
    }
}

impl const Ceil for isize {
    fn ceil(self) -> Self {
        self
    }
}

// Floating-point
impl const Ceil for f32 {
    fn ceil(self) -> Self {
        f32::ceil(self)
    }
}

impl const Ceil for f64 {
    fn ceil(self) -> Self {
        f64::ceil(self)
    }
}
