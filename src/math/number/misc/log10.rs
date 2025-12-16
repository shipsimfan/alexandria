/// Computes the base 10 logarithm of a value
pub const trait Log10 {
    /// Computes the base 10 logarithm of a value
    fn log10(self) -> Self;
}

// Unsigned integers

impl const Log10 for u8 {
    fn log10(self) -> Self {
        self.ilog10() as u8
    }
}

impl const Log10 for u16 {
    fn log10(self) -> Self {
        self.ilog10() as u16
    }
}

impl const Log10 for u32 {
    fn log10(self) -> Self {
        self.ilog10()
    }
}

impl const Log10 for u64 {
    fn log10(self) -> Self {
        self.ilog10() as u64
    }
}

impl const Log10 for u128 {
    fn log10(self) -> Self {
        self.ilog10() as u128
    }
}

impl const Log10 for usize {
    fn log10(self) -> Self {
        self.ilog10() as usize
    }
}

// Signed integers

impl const Log10 for i8 {
    fn log10(self) -> Self {
        self.ilog10() as i8
    }
}

impl const Log10 for i16 {
    fn log10(self) -> Self {
        self.ilog10() as i16
    }
}

impl const Log10 for i32 {
    fn log10(self) -> Self {
        self.ilog10() as i32
    }
}

impl const Log10 for i64 {
    fn log10(self) -> Self {
        self.ilog10() as i64
    }
}

impl const Log10 for i128 {
    fn log10(self) -> Self {
        self.ilog10() as i128
    }
}

impl const Log10 for isize {
    fn log10(self) -> Self {
        self.ilog10() as isize
    }
}

// Floating-point
impl Log10 for f32 {
    fn log10(self) -> Self {
        f32::log10(self)
    }
}

impl Log10 for f64 {
    fn log10(self) -> Self {
        f64::log10(self)
    }
}
