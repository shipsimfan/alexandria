/// Copies the sign from a different value
pub const trait CopySign {
    /// Copies the sign from `sign`
    fn copysign(self, sign: Self) -> Self;
}

// Unsigned integers

const impl CopySign for u8 {
    fn copysign(self, _: Self) -> Self {
        self
    }
}

const impl CopySign for u16 {
    fn copysign(self, _: Self) -> Self {
        self
    }
}

const impl CopySign for u32 {
    fn copysign(self, _: Self) -> Self {
        self
    }
}

const impl CopySign for u64 {
    fn copysign(self, _: Self) -> Self {
        self
    }
}

const impl CopySign for u128 {
    fn copysign(self, _: Self) -> Self {
        self
    }
}

const impl CopySign for usize {
    fn copysign(self, _: Self) -> Self {
        self
    }
}

// Signed integers

const impl CopySign for i8 {
    fn copysign(self, sign: Self) -> Self {
        if sign < 0 { -self.abs() } else { self.abs() }
    }
}

const impl CopySign for i16 {
    fn copysign(self, sign: Self) -> Self {
        if sign < 0 { -self.abs() } else { self.abs() }
    }
}

const impl CopySign for i32 {
    fn copysign(self, sign: Self) -> Self {
        if sign < 0 { -self.abs() } else { self.abs() }
    }
}

const impl CopySign for i64 {
    fn copysign(self, sign: Self) -> Self {
        if sign < 0 { -self.abs() } else { self.abs() }
    }
}

const impl CopySign for i128 {
    fn copysign(self, sign: Self) -> Self {
        if sign < 0 { -self.abs() } else { self.abs() }
    }
}

const impl CopySign for isize {
    fn copysign(self, sign: Self) -> Self {
        if sign < 0 { -self.abs() } else { self.abs() }
    }
}

// Floating-point
const impl CopySign for f32 {
    fn copysign(self, sign: Self) -> Self {
        f32::copysign(self, sign)
    }
}

const impl CopySign for f64 {
    fn copysign(self, sign: Self) -> Self {
        f64::copysign(self, sign)
    }
}
