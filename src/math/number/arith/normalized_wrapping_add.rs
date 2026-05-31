/// A trait for types that can perform a normalized wrapping addition operation
pub const trait NormalizedWrappingAdd: Sized {
    /// Add two values together, wrapping around to keep the values within this types normalized
    /// range
    fn normalized_wrapping_add(self, other: Self) -> Self;
}

// Unsigned integers

impl const NormalizedWrappingAdd for u8 {
    fn normalized_wrapping_add(self, other: Self) -> Self {
        self.wrapping_add(other)
    }
}

impl const NormalizedWrappingAdd for u16 {
    fn normalized_wrapping_add(self, other: Self) -> Self {
        self.wrapping_add(other)
    }
}

impl const NormalizedWrappingAdd for u32 {
    fn normalized_wrapping_add(self, other: Self) -> Self {
        self.wrapping_add(other)
    }
}

impl const NormalizedWrappingAdd for u64 {
    fn normalized_wrapping_add(self, other: Self) -> Self {
        self.wrapping_add(other)
    }
}

impl const NormalizedWrappingAdd for u128 {
    fn normalized_wrapping_add(self, other: Self) -> Self {
        self.wrapping_add(other)
    }
}

impl const NormalizedWrappingAdd for usize {
    fn normalized_wrapping_add(self, other: Self) -> Self {
        self.wrapping_add(other)
    }
}

// Signed integers

impl const NormalizedWrappingAdd for i8 {
    fn normalized_wrapping_add(self, other: Self) -> Self {
        self.wrapping_add(other) & i8::MIN
    }
}

impl const NormalizedWrappingAdd for i16 {
    fn normalized_wrapping_add(self, other: Self) -> Self {
        self.wrapping_add(other) & i16::MIN
    }
}

impl const NormalizedWrappingAdd for i32 {
    fn normalized_wrapping_add(self, other: Self) -> Self {
        self.wrapping_add(other) & i32::MIN
    }
}

impl const NormalizedWrappingAdd for i64 {
    fn normalized_wrapping_add(self, other: Self) -> Self {
        self.wrapping_add(other) & i64::MIN
    }
}

impl const NormalizedWrappingAdd for i128 {
    fn normalized_wrapping_add(self, other: Self) -> Self {
        self.wrapping_add(other) & i128::MIN
    }
}

impl const NormalizedWrappingAdd for isize {
    fn normalized_wrapping_add(self, other: Self) -> Self {
        self.wrapping_add(other) & isize::MIN
    }
}

// Floating point numbers

impl const NormalizedWrappingAdd for f32 {
    fn normalized_wrapping_add(self, other: Self) -> Self {
        (self + other).fract()
    }
}

impl const NormalizedWrappingAdd for f64 {
    fn normalized_wrapping_add(self, other: Self) -> Self {
        (self + other).fract()
    }
}
