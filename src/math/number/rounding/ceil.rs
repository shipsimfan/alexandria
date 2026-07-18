/// Rounds a value up to the nearest integer
pub const trait Ceil {
    /// Rounds a value up to the nearest integer
    fn ceil(self) -> Self;
}

// Unsigned integers

const impl Ceil for u8 {
    fn ceil(self) -> Self {
        self
    }
}

const impl Ceil for u16 {
    fn ceil(self) -> Self {
        self
    }
}

const impl Ceil for u32 {
    fn ceil(self) -> Self {
        self
    }
}

const impl Ceil for u64 {
    fn ceil(self) -> Self {
        self
    }
}

const impl Ceil for u128 {
    fn ceil(self) -> Self {
        self
    }
}

const impl Ceil for usize {
    fn ceil(self) -> Self {
        self
    }
}

// Signed integers

const impl Ceil for i8 {
    fn ceil(self) -> Self {
        self
    }
}

const impl Ceil for i16 {
    fn ceil(self) -> Self {
        self
    }
}

const impl Ceil for i32 {
    fn ceil(self) -> Self {
        self
    }
}

const impl Ceil for i64 {
    fn ceil(self) -> Self {
        self
    }
}

const impl Ceil for i128 {
    fn ceil(self) -> Self {
        self
    }
}

const impl Ceil for isize {
    fn ceil(self) -> Self {
        self
    }
}

// Floating-point
const impl Ceil for f32 {
    fn ceil(self) -> Self {
        f32::ceil(self)
    }
}

const impl Ceil for f64 {
    fn ceil(self) -> Self {
        f64::ceil(self)
    }
}
