/// Rounds a value to the nearest integer
pub const trait Round {
    /// Rounds a value to the nearest integer
    fn round(self) -> Self;
}

// Unsigned integers

const impl Round for u8 {
    fn round(self) -> Self {
        self
    }
}

const impl Round for u16 {
    fn round(self) -> Self {
        self
    }
}

const impl Round for u32 {
    fn round(self) -> Self {
        self
    }
}

const impl Round for u64 {
    fn round(self) -> Self {
        self
    }
}

const impl Round for u128 {
    fn round(self) -> Self {
        self
    }
}

const impl Round for usize {
    fn round(self) -> Self {
        self
    }
}

// Signed integers

const impl Round for i8 {
    fn round(self) -> Self {
        self
    }
}

const impl Round for i16 {
    fn round(self) -> Self {
        self
    }
}

const impl Round for i32 {
    fn round(self) -> Self {
        self
    }
}

const impl Round for i64 {
    fn round(self) -> Self {
        self
    }
}

const impl Round for i128 {
    fn round(self) -> Self {
        self
    }
}

const impl Round for isize {
    fn round(self) -> Self {
        self
    }
}

// Floating-point
const impl Round for f32 {
    fn round(self) -> Self {
        f32::round(self)
    }
}

const impl Round for f64 {
    fn round(self) -> Self {
        f64::round(self)
    }
}
