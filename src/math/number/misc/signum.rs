/// Returns a number that represents the sign of `self`
pub const trait Signum {
    /// Returns a number that represents the sign of `self`
    fn signum(self) -> Self;
}

// Unsigned integers

const impl Signum for u8 {
    fn signum(self) -> Self {
        1
    }
}

const impl Signum for u16 {
    fn signum(self) -> Self {
        1
    }
}

const impl Signum for u32 {
    fn signum(self) -> Self {
        1
    }
}

const impl Signum for u64 {
    fn signum(self) -> Self {
        1
    }
}

const impl Signum for u128 {
    fn signum(self) -> Self {
        1
    }
}

const impl Signum for usize {
    fn signum(self) -> Self {
        1
    }
}

// Signed integers

const impl Signum for i8 {
    fn signum(self) -> Self {
        i8::signum(self)
    }
}

const impl Signum for i16 {
    fn signum(self) -> Self {
        i16::signum(self)
    }
}

const impl Signum for i32 {
    fn signum(self) -> Self {
        i32::signum(self)
    }
}

const impl Signum for i64 {
    fn signum(self) -> Self {
        i64::signum(self)
    }
}

const impl Signum for i128 {
    fn signum(self) -> Self {
        i128::signum(self)
    }
}

const impl Signum for isize {
    fn signum(self) -> Self {
        isize::signum(self)
    }
}

// Floating-point
const impl Signum for f32 {
    fn signum(self) -> Self {
        f32::signum(self)
    }
}

const impl Signum for f64 {
    fn signum(self) -> Self {
        f64::signum(self)
    }
}
