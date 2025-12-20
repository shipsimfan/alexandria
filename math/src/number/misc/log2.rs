/// Computes the base 2 logarithm of a value
pub const trait Log2 {
    /// Computes the base 2 logarithm of a value
    fn log2(self) -> Self;
}

// Unsigned integers

impl const Log2 for u8 {
    fn log2(self) -> Self {
        self.ilog2() as u8
    }
}

impl const Log2 for u16 {
    fn log2(self) -> Self {
        self.ilog2() as u16
    }
}

impl const Log2 for u32 {
    fn log2(self) -> Self {
        self.ilog2()
    }
}

impl const Log2 for u64 {
    fn log2(self) -> Self {
        self.ilog2() as u64
    }
}

impl const Log2 for u128 {
    fn log2(self) -> Self {
        self.ilog2() as u128
    }
}

impl const Log2 for usize {
    fn log2(self) -> Self {
        self.ilog2() as usize
    }
}

// Signed integers

impl const Log2 for i8 {
    fn log2(self) -> Self {
        self.ilog2() as i8
    }
}

impl const Log2 for i16 {
    fn log2(self) -> Self {
        self.ilog2() as i16
    }
}

impl const Log2 for i32 {
    fn log2(self) -> Self {
        self.ilog2() as i32
    }
}

impl const Log2 for i64 {
    fn log2(self) -> Self {
        self.ilog2() as i64
    }
}

impl const Log2 for i128 {
    fn log2(self) -> Self {
        self.ilog2() as i128
    }
}

impl const Log2 for isize {
    fn log2(self) -> Self {
        self.ilog2() as isize
    }
}

// Floating-point
impl Log2 for f32 {
    fn log2(self) -> Self {
        f32::log2(self)
    }
}

impl Log2 for f64 {
    fn log2(self) -> Self {
        f64::log2(self)
    }
}
