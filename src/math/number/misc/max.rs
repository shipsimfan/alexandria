/// A value which can be compared for maximum
pub const trait Max: Sized {
    /// Returns the maximum of `self` and `other`
    fn max(self, other: Self) -> Self;
}

// Unsigned integers

const impl Max for u8 {
    fn max(self, other: Self) -> Self {
        if self >= other { self } else { other }
    }
}

const impl Max for u16 {
    fn max(self, other: Self) -> Self {
        if self >= other { self } else { other }
    }
}

const impl Max for u32 {
    fn max(self, other: Self) -> Self {
        if self >= other { self } else { other }
    }
}

const impl Max for u64 {
    fn max(self, other: Self) -> Self {
        if self >= other { self } else { other }
    }
}

const impl Max for u128 {
    fn max(self, other: Self) -> Self {
        if self >= other { self } else { other }
    }
}

const impl Max for usize {
    fn max(self, other: Self) -> Self {
        if self >= other { self } else { other }
    }
}

// Signed integers

const impl Max for i8 {
    fn max(self, other: Self) -> Self {
        if self >= other { self } else { other }
    }
}

const impl Max for i16 {
    fn max(self, other: Self) -> Self {
        if self >= other { self } else { other }
    }
}

const impl Max for i32 {
    fn max(self, other: Self) -> Self {
        if self >= other { self } else { other }
    }
}

const impl Max for i64 {
    fn max(self, other: Self) -> Self {
        if self >= other { self } else { other }
    }
}

const impl Max for i128 {
    fn max(self, other: Self) -> Self {
        if self >= other { self } else { other }
    }
}

const impl Max for isize {
    fn max(self, other: Self) -> Self {
        if self >= other { self } else { other }
    }
}

// Floating-point numbers

const impl Max for f32 {
    fn max(self, other: Self) -> Self {
        if self >= other { self } else { other }
    }
}

const impl Max for f64 {
    fn max(self, other: Self) -> Self {
        if self >= other { self } else { other }
    }
}
