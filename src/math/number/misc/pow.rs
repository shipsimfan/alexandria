/// Raises a value to another value
pub const trait Pow {
    /// Raises a value to another value
    fn pow(self, n: Self) -> Self;
}

// Unsigned integers

impl const Pow for u8 {
    fn pow(self, n: Self) -> Self {
        self.pow(n as _)
    }
}

impl const Pow for u16 {
    fn pow(self, n: Self) -> Self {
        self.pow(n as _)
    }
}

impl const Pow for u32 {
    fn pow(self, n: Self) -> Self {
        self.pow(n)
    }
}

impl const Pow for u64 {
    fn pow(self, n: Self) -> Self {
        self.pow(n as _)
    }
}

impl const Pow for u128 {
    fn pow(self, n: Self) -> Self {
        self.pow(n as _)
    }
}

impl const Pow for usize {
    fn pow(self, n: Self) -> Self {
        self.pow(n as _)
    }
}

// Signed integers

impl const Pow for i8 {
    fn pow(self, n: Self) -> Self {
        self.pow(n as _)
    }
}

impl const Pow for i16 {
    fn pow(self, n: Self) -> Self {
        self.pow(n as _)
    }
}

impl const Pow for i32 {
    fn pow(self, n: Self) -> Self {
        self.pow(n as _)
    }
}

impl const Pow for i64 {
    fn pow(self, n: Self) -> Self {
        self.pow(n as _)
    }
}

impl const Pow for i128 {
    fn pow(self, n: Self) -> Self {
        self.pow(n as _)
    }
}

impl const Pow for isize {
    fn pow(self, n: Self) -> Self {
        self.pow(n as _)
    }
}

// Floating-point
impl Pow for f32 {
    fn pow(self, n: Self) -> Self {
        f32::powf(self, n)
    }
}

impl Pow for f64 {
    fn pow(self, n: Self) -> Self {
        f64::powf(self, n)
    }
}
