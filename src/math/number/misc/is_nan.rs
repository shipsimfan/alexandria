/// Returns `true` if this value is NaN
pub const trait IsNaN {
    /// Returns `true` if this value is NaN
    fn is_nan(&self) -> bool;
}

// Unsigned integers

const impl IsNaN for u8 {
    fn is_nan(&self) -> bool {
        false
    }
}

const impl IsNaN for u16 {
    fn is_nan(&self) -> bool {
        false
    }
}

const impl IsNaN for u32 {
    fn is_nan(&self) -> bool {
        false
    }
}

const impl IsNaN for u64 {
    fn is_nan(&self) -> bool {
        false
    }
}

const impl IsNaN for u128 {
    fn is_nan(&self) -> bool {
        false
    }
}

const impl IsNaN for usize {
    fn is_nan(&self) -> bool {
        false
    }
}

// Signed integers

const impl IsNaN for i8 {
    fn is_nan(&self) -> bool {
        false
    }
}

const impl IsNaN for i16 {
    fn is_nan(&self) -> bool {
        false
    }
}

const impl IsNaN for i32 {
    fn is_nan(&self) -> bool {
        false
    }
}

const impl IsNaN for i64 {
    fn is_nan(&self) -> bool {
        false
    }
}

const impl IsNaN for i128 {
    fn is_nan(&self) -> bool {
        false
    }
}

const impl IsNaN for isize {
    fn is_nan(&self) -> bool {
        false
    }
}

// Floating-point
const impl IsNaN for f32 {
    fn is_nan(&self) -> bool {
        f32::is_nan(*self)
    }
}

const impl IsNaN for f64 {
    fn is_nan(&self) -> bool {
        f64::is_nan(*self)
    }
}
