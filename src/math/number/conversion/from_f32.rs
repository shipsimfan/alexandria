/// A value that can be (potentially lossily) created from an [`f32`]
pub const trait FromF32 {
    /// Creates `Self` from an [`f32`]
    fn from_f32(value: f32) -> Self;
}

// Unsigned integers

impl const FromF32 for u8 {
    fn from_f32(value: f32) -> Self {
        value as u8
    }
}

impl const FromF32 for u16 {
    fn from_f32(value: f32) -> Self {
        value as u16
    }
}

impl const FromF32 for u32 {
    fn from_f32(value: f32) -> Self {
        value as u32
    }
}

impl const FromF32 for u64 {
    fn from_f32(value: f32) -> Self {
        value as u64
    }
}

impl const FromF32 for u128 {
    fn from_f32(value: f32) -> Self {
        value as u128
    }
}

impl const FromF32 for usize {
    fn from_f32(value: f32) -> Self {
        value as usize
    }
}

// Signed integers

impl const FromF32 for i8 {
    fn from_f32(value: f32) -> Self {
        value as i8
    }
}

impl const FromF32 for i16 {
    fn from_f32(value: f32) -> Self {
        value as i16
    }
}

impl const FromF32 for i32 {
    fn from_f32(value: f32) -> Self {
        value as i32
    }
}

impl const FromF32 for i64 {
    fn from_f32(value: f32) -> Self {
        value as i64
    }
}

impl const FromF32 for i128 {
    fn from_f32(value: f32) -> Self {
        value as i128
    }
}

impl const FromF32 for isize {
    fn from_f32(value: f32) -> Self {
        value as isize
    }
}

// Floating-point

impl const FromF32 for f32 {
    fn from_f32(value: f32) -> Self {
        value
    }
}

impl const FromF32 for f64 {
    fn from_f32(value: f32) -> Self {
        value as Self
    }
}
