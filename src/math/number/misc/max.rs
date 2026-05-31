/// A value which can be compared for maximum
pub const trait Max: Sized {
    /// Returns the maximum of `self` and `other`
    fn max(self, other: Self) -> Self;
}

// Unsigned integers

impl const Max for u8 {
    fn max(self, other: Self) -> Self {
        if self >= other { self } else { other }
    }
}

impl const Max for u16 {
    fn max(self, other: Self) -> Self {
        if self >= other { self } else { other }
    }
}

impl const Max for u32 {
    fn max(self, other: Self) -> Self {
        if self >= other { self } else { other }
    }
}

impl const Max for u64 {
    fn max(self, other: Self) -> Self {
        if self >= other { self } else { other }
    }
}

impl const Max for u128 {
    fn max(self, other: Self) -> Self {
        if self >= other { self } else { other }
    }
}

impl const Max for usize {
    fn max(self, other: Self) -> Self {
        if self >= other { self } else { other }
    }
}

// Signed integers

impl const Max for i8 {
    fn max(self, other: Self) -> Self {
        if self >= other { self } else { other }
    }
}

impl const Max for i16 {
    fn max(self, other: Self) -> Self {
        if self >= other { self } else { other }
    }
}

impl const Max for i32 {
    fn max(self, other: Self) -> Self {
        if self >= other { self } else { other }
    }
}

impl const Max for i64 {
    fn max(self, other: Self) -> Self {
        if self >= other { self } else { other }
    }
}

impl const Max for i128 {
    fn max(self, other: Self) -> Self {
        if self >= other { self } else { other }
    }
}

impl const Max for isize {
    fn max(self, other: Self) -> Self {
        if self >= other { self } else { other }
    }
}

// Floating-point numbers

impl const Max for f32 {
    fn max(self, other: Self) -> Self {
        if self >= other { self } else { other }
    }
}

impl const Max for f64 {
    fn max(self, other: Self) -> Self {
        if self >= other { self } else { other }
    }
}
