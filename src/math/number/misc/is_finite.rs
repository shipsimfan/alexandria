/// Returns `true` if this number is neither infinite nor NaN
pub const trait IsFinite {
    /// Returns `true` if this number is neither infinite nor NaN
    fn is_finite(&self) -> bool;
}

// Unsigned integers

const impl IsFinite for u8 {
    fn is_finite(&self) -> bool {
        true
    }
}

const impl IsFinite for u16 {
    fn is_finite(&self) -> bool {
        true
    }
}

const impl IsFinite for u32 {
    fn is_finite(&self) -> bool {
        true
    }
}

const impl IsFinite for u64 {
    fn is_finite(&self) -> bool {
        true
    }
}

const impl IsFinite for u128 {
    fn is_finite(&self) -> bool {
        true
    }
}

const impl IsFinite for usize {
    fn is_finite(&self) -> bool {
        true
    }
}

// Signed integers

const impl IsFinite for i8 {
    fn is_finite(&self) -> bool {
        true
    }
}

const impl IsFinite for i16 {
    fn is_finite(&self) -> bool {
        true
    }
}

const impl IsFinite for i32 {
    fn is_finite(&self) -> bool {
        true
    }
}

const impl IsFinite for i64 {
    fn is_finite(&self) -> bool {
        true
    }
}

const impl IsFinite for i128 {
    fn is_finite(&self) -> bool {
        true
    }
}

const impl IsFinite for isize {
    fn is_finite(&self) -> bool {
        true
    }
}

// Floating-point
const impl IsFinite for f32 {
    fn is_finite(&self) -> bool {
        f32::is_finite(*self)
    }
}

const impl IsFinite for f64 {
    fn is_finite(&self) -> bool {
        f64::is_finite(*self)
    }
}
