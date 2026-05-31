/// A trait for types that can perform a normalized wrapping subtraction operation
pub const trait NormalizedWrappingSub: Sized {
    /// Subtract two values together, wrapping around to keep the values within this types
    /// normalized range
    fn normalized_wrapping_sub(self, other: Self) -> Self;
}

// Unsigned integers

impl const NormalizedWrappingSub for u8 {
    fn normalized_wrapping_sub(self, other: Self) -> Self {
        self.wrapping_sub(other)
    }
}

impl const NormalizedWrappingSub for u16 {
    fn normalized_wrapping_sub(self, other: Self) -> Self {
        self.wrapping_sub(other)
    }
}

impl const NormalizedWrappingSub for u32 {
    fn normalized_wrapping_sub(self, other: Self) -> Self {
        self.wrapping_sub(other)
    }
}

impl const NormalizedWrappingSub for u64 {
    fn normalized_wrapping_sub(self, other: Self) -> Self {
        self.wrapping_sub(other)
    }
}

impl const NormalizedWrappingSub for u128 {
    fn normalized_wrapping_sub(self, other: Self) -> Self {
        self.wrapping_sub(other)
    }
}

impl const NormalizedWrappingSub for usize {
    fn normalized_wrapping_sub(self, other: Self) -> Self {
        self.wrapping_sub(other)
    }
}

// Signed integers

impl const NormalizedWrappingSub for i8 {
    fn normalized_wrapping_sub(self, other: Self) -> Self {
        self.wrapping_sub(other) & i8::MIN
    }
}

impl const NormalizedWrappingSub for i16 {
    fn normalized_wrapping_sub(self, other: Self) -> Self {
        self.wrapping_sub(other) & i16::MIN
    }
}

impl const NormalizedWrappingSub for i32 {
    fn normalized_wrapping_sub(self, other: Self) -> Self {
        self.wrapping_sub(other) & i32::MIN
    }
}

impl const NormalizedWrappingSub for i64 {
    fn normalized_wrapping_sub(self, other: Self) -> Self {
        self.wrapping_sub(other) & i64::MIN
    }
}

impl const NormalizedWrappingSub for i128 {
    fn normalized_wrapping_sub(self, other: Self) -> Self {
        self.wrapping_sub(other) & i128::MIN
    }
}

impl const NormalizedWrappingSub for isize {
    fn normalized_wrapping_sub(self, other: Self) -> Self {
        self.wrapping_sub(other) & isize::MIN
    }
}

// Floating point numbers

impl const NormalizedWrappingSub for f32 {
    fn normalized_wrapping_sub(self, other: Self) -> Self {
        (self - other).fract()
    }
}

impl const NormalizedWrappingSub for f64 {
    fn normalized_wrapping_sub(self, other: Self) -> Self {
        (self - other).fract()
    }
}
