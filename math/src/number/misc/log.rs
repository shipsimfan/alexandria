/// Computes the logarithm of a value with respect to an arbitrary base
pub const trait Log: Sized {
    /// The type a base can be
    type Base = Self;

    /// Computes the logarithm of a value with respect to an arbitrary base
    fn log(self, base: Self::Base) -> Self;
}

// Unsigned integers

impl const Log for u8 {
    fn log(self, base: Self) -> Self {
        self.ilog(base) as u8
    }
}

impl const Log for u16 {
    fn log(self, base: Self) -> Self {
        self.ilog(base) as u16
    }
}

impl const Log for u32 {
    fn log(self, base: Self) -> Self {
        self.ilog(base)
    }
}

impl const Log for u64 {
    fn log(self, base: Self) -> Self {
        self.ilog(base) as u64
    }
}

impl const Log for u128 {
    fn log(self, base: Self) -> Self {
        self.ilog(base) as u128
    }
}

impl const Log for usize {
    fn log(self, base: Self) -> Self {
        self.ilog(base) as usize
    }
}

// Signed integers

impl const Log for i8 {
    fn log(self, base: Self) -> Self {
        self.ilog(base) as i8
    }
}

impl const Log for i16 {
    fn log(self, base: Self) -> Self {
        self.ilog(base) as i16
    }
}

impl const Log for i32 {
    fn log(self, base: Self) -> Self {
        self.ilog(base) as i32
    }
}

impl const Log for i64 {
    fn log(self, base: Self) -> Self {
        self.ilog(base) as i64
    }
}

impl const Log for i128 {
    fn log(self, base: Self) -> Self {
        self.ilog(base) as i128
    }
}

impl const Log for isize {
    fn log(self, base: Self) -> Self {
        self.ilog(base) as isize
    }
}

// Floating-point
impl Log for f32 {
    fn log(self, base: Self) -> Self {
        f32::log(self, base)
    }
}

impl Log for f64 {
    fn log(self, base: Self) -> Self {
        f64::log(self, base)
    }
}
