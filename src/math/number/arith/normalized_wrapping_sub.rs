/// A trait for types that can perform a normalized wrapping subtraction operation
pub const trait NormalizedWrappingSub: Sized {
    /// Subtract two values together, wrapping around to keep the values within this types
    /// normalized range
    fn normalized_wrapping_sub(self, other: Self) -> Self;
}

// Unsigned integers

const impl NormalizedWrappingSub for u8 {
    fn normalized_wrapping_sub(self, other: Self) -> Self {
        self.wrapping_sub(other)
    }
}

const impl NormalizedWrappingSub for u16 {
    fn normalized_wrapping_sub(self, other: Self) -> Self {
        self.wrapping_sub(other)
    }
}

const impl NormalizedWrappingSub for u32 {
    fn normalized_wrapping_sub(self, other: Self) -> Self {
        self.wrapping_sub(other)
    }
}

const impl NormalizedWrappingSub for u64 {
    fn normalized_wrapping_sub(self, other: Self) -> Self {
        self.wrapping_sub(other)
    }
}

const impl NormalizedWrappingSub for u128 {
    fn normalized_wrapping_sub(self, other: Self) -> Self {
        self.wrapping_sub(other)
    }
}

const impl NormalizedWrappingSub for usize {
    fn normalized_wrapping_sub(self, other: Self) -> Self {
        self.wrapping_sub(other)
    }
}

// Signed integers

const impl NormalizedWrappingSub for i8 {
    fn normalized_wrapping_sub(self, other: Self) -> Self {
        self.wrapping_sub(other) & i8::MIN
    }
}

const impl NormalizedWrappingSub for i16 {
    fn normalized_wrapping_sub(self, other: Self) -> Self {
        self.wrapping_sub(other) & i16::MIN
    }
}

const impl NormalizedWrappingSub for i32 {
    fn normalized_wrapping_sub(self, other: Self) -> Self {
        self.wrapping_sub(other) & i32::MIN
    }
}

const impl NormalizedWrappingSub for i64 {
    fn normalized_wrapping_sub(self, other: Self) -> Self {
        self.wrapping_sub(other) & i64::MIN
    }
}

const impl NormalizedWrappingSub for i128 {
    fn normalized_wrapping_sub(self, other: Self) -> Self {
        self.wrapping_sub(other) & i128::MIN
    }
}

const impl NormalizedWrappingSub for isize {
    fn normalized_wrapping_sub(self, other: Self) -> Self {
        self.wrapping_sub(other) & isize::MIN
    }
}

// Floating point numbers

const impl NormalizedWrappingSub for f32 {
    fn normalized_wrapping_sub(self, other: Self) -> Self {
        let fract = (self - other).fract();
        if fract < 0.0 { fract + 1.0 } else { fract }
    }
}

const impl NormalizedWrappingSub for f64 {
    fn normalized_wrapping_sub(self, other: Self) -> Self {
        let fract = (self - other).fract();
        if fract < 0.0 { fract + 1.0 } else { fract }
    }
}
