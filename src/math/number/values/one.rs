/// Defines a one value for a type
pub trait One {
    /// The one value of this type
    const ONE: Self;

    /// The one value of this when using a normalized range (full positive range = 0-1)
    const NORMALIZED_ONE: Self;
}

// Unsigned integers

impl One for u8 {
    const ONE: Self = 1;
    const NORMALIZED_ONE: Self = u8::MAX;
}

impl One for u16 {
    const ONE: Self = 1;
    const NORMALIZED_ONE: Self = u16::MAX;
}

impl One for u32 {
    const ONE: Self = 1;
    const NORMALIZED_ONE: Self = u32::MAX;
}

impl One for u64 {
    const ONE: Self = 1;
    const NORMALIZED_ONE: Self = u64::MAX;
}

impl One for u128 {
    const ONE: Self = 1;
    const NORMALIZED_ONE: Self = u128::MAX;
}

impl One for usize {
    const ONE: Self = 1;
    const NORMALIZED_ONE: Self = usize::MAX;
}

// Signed integers

impl One for i8 {
    const ONE: Self = 1;
    const NORMALIZED_ONE: Self = i8::MAX;
}

impl One for i16 {
    const ONE: Self = 1;
    const NORMALIZED_ONE: Self = i16::MAX;
}

impl One for i32 {
    const ONE: Self = 1;
    const NORMALIZED_ONE: Self = i32::MAX;
}

impl One for i64 {
    const ONE: Self = 1;
    const NORMALIZED_ONE: Self = i64::MAX;
}

impl One for i128 {
    const ONE: Self = 1;
    const NORMALIZED_ONE: Self = i128::MAX;
}

impl One for isize {
    const ONE: Self = 1;
    const NORMALIZED_ONE: Self = isize::MAX;
}

// Floating-point

impl One for f32 {
    const ONE: Self = 1.0;
    const NORMALIZED_ONE: Self = 1.0;
}

impl One for f64 {
    const ONE: Self = 1.0;
    const NORMALIZED_ONE: Self = 1.0;
}
