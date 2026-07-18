/// Raises a value to another value
pub const trait PowI {
    /// Raises a value to another value
    fn powi(self, n: u32) -> Self;
}

// Unsigned integers

const impl PowI for u8 {
    fn powi(self, n: u32) -> Self {
        self.pow(n)
    }
}

const impl PowI for u16 {
    fn powi(self, n: u32) -> Self {
        self.pow(n)
    }
}

const impl PowI for u32 {
    fn powi(self, n: u32) -> Self {
        self.pow(n)
    }
}

const impl PowI for u64 {
    fn powi(self, n: u32) -> Self {
        self.pow(n)
    }
}

const impl PowI for u128 {
    fn powi(self, n: u32) -> Self {
        self.pow(n)
    }
}

const impl PowI for usize {
    fn powi(self, n: u32) -> Self {
        self.pow(n)
    }
}

// Signed integers

const impl PowI for i8 {
    fn powi(self, n: u32) -> Self {
        self.pow(n)
    }
}

const impl PowI for i16 {
    fn powi(self, n: u32) -> Self {
        self.pow(n)
    }
}

const impl PowI for i32 {
    fn powi(self, n: u32) -> Self {
        self.pow(n)
    }
}

const impl PowI for i64 {
    fn powi(self, n: u32) -> Self {
        self.pow(n)
    }
}

const impl PowI for i128 {
    fn powi(self, n: u32) -> Self {
        self.pow(n)
    }
}

const impl PowI for isize {
    fn powi(self, n: u32) -> Self {
        self.pow(n)
    }
}

// Floating-point
impl PowI for f32 {
    fn powi(self, n: u32) -> Self {
        f32::powi(self, n as i32)
    }
}

impl PowI for f64 {
    fn powi(self, n: u32) -> Self {
        f64::powi(self, n as i32)
    }
}
