/// Returns `true` if this value is positive infinity or negative infinity, and `false` otherwise
pub const trait IsInfinite {
    /// Returns `true` if this value is positive infinity or negative infinity, and `false`
    /// otherwise.
    fn is_infinite(&self) -> bool;
}

// Unsigned integers

const impl IsInfinite for u8 {
    fn is_infinite(&self) -> bool {
        false
    }
}

const impl IsInfinite for u16 {
    fn is_infinite(&self) -> bool {
        false
    }
}

const impl IsInfinite for u32 {
    fn is_infinite(&self) -> bool {
        false
    }
}

const impl IsInfinite for u64 {
    fn is_infinite(&self) -> bool {
        false
    }
}

const impl IsInfinite for u128 {
    fn is_infinite(&self) -> bool {
        false
    }
}

const impl IsInfinite for usize {
    fn is_infinite(&self) -> bool {
        false
    }
}

// Signed integers

const impl IsInfinite for i8 {
    fn is_infinite(&self) -> bool {
        false
    }
}

const impl IsInfinite for i16 {
    fn is_infinite(&self) -> bool {
        false
    }
}

const impl IsInfinite for i32 {
    fn is_infinite(&self) -> bool {
        false
    }
}

const impl IsInfinite for i64 {
    fn is_infinite(&self) -> bool {
        false
    }
}

const impl IsInfinite for i128 {
    fn is_infinite(&self) -> bool {
        false
    }
}

const impl IsInfinite for isize {
    fn is_infinite(&self) -> bool {
        false
    }
}

// Floating-point
const impl IsInfinite for f32 {
    fn is_infinite(&self) -> bool {
        f32::is_infinite(*self)
    }
}

const impl IsInfinite for f64 {
    fn is_infinite(&self) -> bool {
        f64::is_infinite(*self)
    }
}
