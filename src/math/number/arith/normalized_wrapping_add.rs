/// A trait for types that can perform a normalized wrapping addition operation
pub const trait NormalizedWrappingAdd: Sized {
    /// Add two values together, wrapping around to keep the values within this types normalized
    /// range
    fn normalized_wrapping_add(self, other: Self) -> Self;
}

// Unsigned integers

const impl NormalizedWrappingAdd for u8 {
    fn normalized_wrapping_add(self, other: Self) -> Self {
        self.wrapping_add(other)
    }
}

const impl NormalizedWrappingAdd for u16 {
    fn normalized_wrapping_add(self, other: Self) -> Self {
        self.wrapping_add(other)
    }
}

const impl NormalizedWrappingAdd for u32 {
    fn normalized_wrapping_add(self, other: Self) -> Self {
        self.wrapping_add(other)
    }
}

const impl NormalizedWrappingAdd for u64 {
    fn normalized_wrapping_add(self, other: Self) -> Self {
        self.wrapping_add(other)
    }
}

const impl NormalizedWrappingAdd for u128 {
    fn normalized_wrapping_add(self, other: Self) -> Self {
        self.wrapping_add(other)
    }
}

const impl NormalizedWrappingAdd for usize {
    fn normalized_wrapping_add(self, other: Self) -> Self {
        self.wrapping_add(other)
    }
}

// Signed integers

const impl NormalizedWrappingAdd for i8 {
    fn normalized_wrapping_add(self, other: Self) -> Self {
        self.wrapping_add(other) & i8::MIN
    }
}

const impl NormalizedWrappingAdd for i16 {
    fn normalized_wrapping_add(self, other: Self) -> Self {
        self.wrapping_add(other) & i16::MIN
    }
}

const impl NormalizedWrappingAdd for i32 {
    fn normalized_wrapping_add(self, other: Self) -> Self {
        self.wrapping_add(other) & i32::MIN
    }
}

const impl NormalizedWrappingAdd for i64 {
    fn normalized_wrapping_add(self, other: Self) -> Self {
        self.wrapping_add(other) & i64::MIN
    }
}

const impl NormalizedWrappingAdd for i128 {
    fn normalized_wrapping_add(self, other: Self) -> Self {
        self.wrapping_add(other) & i128::MIN
    }
}

const impl NormalizedWrappingAdd for isize {
    fn normalized_wrapping_add(self, other: Self) -> Self {
        self.wrapping_add(other) & isize::MIN
    }
}

// Floating point numbers

const impl NormalizedWrappingAdd for f32 {
    fn normalized_wrapping_add(self, other: Self) -> Self {
        let fract = (self + other).fract();
        if fract < 0.0 { fract + 1.0 } else { fract }
    }
}

const impl NormalizedWrappingAdd for f64 {
    fn normalized_wrapping_add(self, other: Self) -> Self {
        let fract = (self + other).fract();
        if fract < 0.0 { fract + 1.0 } else { fract }
    }
}
