/// Computes the absolute value
pub const trait Abs {
    /// Computes the absolute value
    fn abs(self) -> Self;
}

// Unsigned integers

impl const Abs for u8 {
    fn abs(self) -> Self {
        self
    }
}

impl const Abs for u16 {
    fn abs(self) -> Self {
        self
    }
}

impl const Abs for u32 {
    fn abs(self) -> Self {
        self
    }
}

impl const Abs for u64 {
    fn abs(self) -> Self {
        self
    }
}

impl const Abs for u128 {
    fn abs(self) -> Self {
        self
    }
}

impl const Abs for usize {
    fn abs(self) -> Self {
        self
    }
}

// Signed integers

impl const Abs for i8 {
    fn abs(self) -> Self {
        i8::abs(self)
    }
}

impl const Abs for i16 {
    fn abs(self) -> Self {
        i16::abs(self)
    }
}

impl const Abs for i32 {
    fn abs(self) -> Self {
        i32::abs(self)
    }
}

impl const Abs for i64 {
    fn abs(self) -> Self {
        i64::abs(self)
    }
}

impl const Abs for i128 {
    fn abs(self) -> Self {
        i128::abs(self)
    }
}

impl const Abs for isize {
    fn abs(self) -> Self {
        isize::abs(self)
    }
}

// Floating-point
impl const Abs for f32 {
    fn abs(self) -> Self {
        f32::abs(self)
    }
}

impl const Abs for f64 {
    fn abs(self) -> Self {
        f64::abs(self)
    }
}
