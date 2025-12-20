/// Returns a number that represents the sign of `self`
pub const trait CopySign {
    /// Returns a number that represents the sign of `self`
    fn copysign(self, sign: Self) -> Self;
}

// Unsigned integers

impl const CopySign for u8 {
    fn copysign(self, _: Self) -> Self {
        self
    }
}

impl const CopySign for u16 {
    fn copysign(self, _: Self) -> Self {
        self
    }
}

impl const CopySign for u32 {
    fn copysign(self, _: Self) -> Self {
        self
    }
}

impl const CopySign for u64 {
    fn copysign(self, _: Self) -> Self {
        self
    }
}

impl const CopySign for u128 {
    fn copysign(self, _: Self) -> Self {
        self
    }
}

impl const CopySign for usize {
    fn copysign(self, _: Self) -> Self {
        self
    }
}

// Signed integers

impl const CopySign for i8 {
    fn copysign(self, sign: Self) -> Self {
        if sign < 0 {
            -self.abs()
        } else {
            self.abs()
        }
    }
}

impl const CopySign for i16 {
    fn copysign(self, sign: Self) -> Self {
        if sign < 0 {
            -self.abs()
        } else {
            self.abs()
        }
    }
}

impl const CopySign for i32 {
    fn copysign(self, sign: Self) -> Self {
        if sign < 0 {
            -self.abs()
        } else {
            self.abs()
        }
    }
}

impl const CopySign for i64 {
    fn copysign(self, sign: Self) -> Self {
        if sign < 0 {
            -self.abs()
        } else {
            self.abs()
        }
    }
}

impl const CopySign for i128 {
    fn copysign(self, sign: Self) -> Self {
        if sign < 0 {
            -self.abs()
        } else {
            self.abs()
        }
    }
}

impl const CopySign for isize {
    fn copysign(self, sign: Self) -> Self {
        if sign < 0 {
            -self.abs()
        } else {
            self.abs()
        }
    }
}

// Floating-point
impl const CopySign for f32 {
    fn copysign(self, sign: Self) -> Self {
        f32::copysign(self, sign)
    }
}

impl const CopySign for f64 {
    fn copysign(self, sign: Self) -> Self {
        f64::copysign(self, sign)
    }
}
