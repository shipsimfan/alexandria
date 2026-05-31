/// A value which can be compared for minimum
pub const trait Min: Sized {
    /// Returns the minimum of `self` and `other`
    fn min(self, other: Self) -> Self;
}

// Unsigned integers

impl const Min for u8 {
    fn min(self, other: Self) -> Self {
        if self <= other { self } else { other }
    }
}

impl const Min for u16 {
    fn min(self, other: Self) -> Self {
        if self <= other { self } else { other }
    }
}

impl const Min for u32 {
    fn min(self, other: Self) -> Self {
        if self <= other { self } else { other }
    }
}

impl const Min for u64 {
    fn min(self, other: Self) -> Self {
        if self <= other { self } else { other }
    }
}

impl const Min for u128 {
    fn min(self, other: Self) -> Self {
        if self <= other { self } else { other }
    }
}

impl const Min for usize {
    fn min(self, other: Self) -> Self {
        if self <= other { self } else { other }
    }
}

// Signed integers

impl const Min for i8 {
    fn min(self, other: Self) -> Self {
        if self <= other { self } else { other }
    }
}

impl const Min for i16 {
    fn min(self, other: Self) -> Self {
        if self <= other { self } else { other }
    }
}

impl const Min for i32 {
    fn min(self, other: Self) -> Self {
        if self <= other { self } else { other }
    }
}

impl const Min for i64 {
    fn min(self, other: Self) -> Self {
        if self <= other { self } else { other }
    }
}

impl const Min for i128 {
    fn min(self, other: Self) -> Self {
        if self <= other { self } else { other }
    }
}

impl const Min for isize {
    fn min(self, other: Self) -> Self {
        if self <= other { self } else { other }
    }
}

// Floating-point numbers

impl const Min for f32 {
    fn min(self, other: Self) -> Self {
        if self <= other { self } else { other }
    }
}

impl const Min for f64 {
    fn min(self, other: Self) -> Self {
        if self <= other { self } else { other }
    }
}
