/// A trait for normalizing values to a specific range
pub const trait Normalize: Sized {
    /// Normalize this value to this types normalized range
    fn normalize(self) -> Self;
}

// Unsigned integers

const impl Normalize for u8 {
    fn normalize(self) -> Self {
        self
    }
}

const impl Normalize for u16 {
    fn normalize(self) -> Self {
        self
    }
}

const impl Normalize for u32 {
    fn normalize(self) -> Self {
        self
    }
}

const impl Normalize for u64 {
    fn normalize(self) -> Self {
        self
    }
}

const impl Normalize for u128 {
    fn normalize(self) -> Self {
        self
    }
}

const impl Normalize for usize {
    fn normalize(self) -> Self {
        self
    }
}

// Signed integers

const impl Normalize for i8 {
    fn normalize(self) -> Self {
        self & i8::MIN
    }
}

const impl Normalize for i16 {
    fn normalize(self) -> Self {
        self & i16::MIN
    }
}

const impl Normalize for i32 {
    fn normalize(self) -> Self {
        self & i32::MIN
    }
}

const impl Normalize for i64 {
    fn normalize(self) -> Self {
        self & i64::MIN
    }
}

const impl Normalize for i128 {
    fn normalize(self) -> Self {
        self & i128::MIN
    }
}

const impl Normalize for isize {
    fn normalize(self) -> Self {
        self & isize::MIN
    }
}

// Floating point numbers

const impl Normalize for f32 {
    fn normalize(self) -> Self {
        self.fract()
    }
}

const impl Normalize for f64 {
    fn normalize(self) -> Self {
        self.fract()
    }
}
