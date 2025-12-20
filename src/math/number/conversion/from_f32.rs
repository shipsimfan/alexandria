/// A value that can be (potentially lossily) created from an [`f32`]
pub const trait FromF32 {
    /// Creates `Self` from an [`f32`]
    fn from_f32(value: f32) -> Self;

    /// Creates `Self` from an [`f32`] in the range [-1.0, 1.0]
    fn from_normalized_f32(value: f32) -> Self;
}

// Unsigned integers

impl const FromF32 for u8 {
    fn from_f32(value: f32) -> Self {
        value as u8
    }

    fn from_normalized_f32(value: f32) -> Self {
        (value * u8::MAX as f32).round() as u8
    }
}

impl const FromF32 for u16 {
    fn from_f32(value: f32) -> Self {
        value as u16
    }

    fn from_normalized_f32(value: f32) -> Self {
        (value * u16::MAX as f32).round() as u16
    }
}

impl const FromF32 for u32 {
    fn from_f32(value: f32) -> Self {
        value as u32
    }

    fn from_normalized_f32(value: f32) -> Self {
        (value * u32::MAX as f32).round() as u32
    }
}

impl const FromF32 for u64 {
    fn from_f32(value: f32) -> Self {
        value as u64
    }

    fn from_normalized_f32(value: f32) -> Self {
        (value * u64::MAX as f32).round() as u64
    }
}

impl const FromF32 for u128 {
    fn from_f32(value: f32) -> Self {
        value as u128
    }

    fn from_normalized_f32(value: f32) -> Self {
        (value * u128::MAX as f32).round() as u128
    }
}

impl const FromF32 for usize {
    fn from_f32(value: f32) -> Self {
        value as usize
    }

    fn from_normalized_f32(value: f32) -> Self {
        (value * usize::MAX as f32).round() as usize
    }
}

// Signed integers

impl const FromF32 for i8 {
    fn from_f32(value: f32) -> Self {
        value as i8
    }

    fn from_normalized_f32(value: f32) -> Self {
        if value == -1.0 {
            i8::MIN
        } else {
            (value * i8::MAX as f32).round() as i8
        }
    }
}

impl const FromF32 for i16 {
    fn from_f32(value: f32) -> Self {
        value as i16
    }

    fn from_normalized_f32(value: f32) -> Self {
        if value == -1.0 {
            i16::MIN
        } else {
            (value * i16::MAX as f32).round() as i16
        }
    }
}

impl const FromF32 for i32 {
    fn from_f32(value: f32) -> Self {
        value as i32
    }

    fn from_normalized_f32(value: f32) -> Self {
        if value == -1.0 {
            i32::MIN
        } else {
            (value * i32::MAX as f32).round() as i32
        }
    }
}

impl const FromF32 for i64 {
    fn from_f32(value: f32) -> Self {
        value as i64
    }

    fn from_normalized_f32(value: f32) -> Self {
        if value == -1.0 {
            i64::MIN
        } else {
            (value * i64::MAX as f32).round() as i64
        }
    }
}

impl const FromF32 for i128 {
    fn from_f32(value: f32) -> Self {
        value as i128
    }

    fn from_normalized_f32(value: f32) -> Self {
        if value == -1.0 {
            i128::MIN
        } else {
            (value * i128::MAX as f32).round() as i128
        }
    }
}

impl const FromF32 for isize {
    fn from_f32(value: f32) -> Self {
        value as isize
    }

    fn from_normalized_f32(value: f32) -> Self {
        if value == -1.0 {
            isize::MIN
        } else {
            (value * isize::MAX as f32).round() as isize
        }
    }
}

// Floating-point

impl const FromF32 for f32 {
    fn from_f32(value: f32) -> Self {
        value
    }

    fn from_normalized_f32(value: f32) -> Self {
        value
    }
}

impl const FromF32 for f64 {
    fn from_f32(value: f32) -> Self {
        value as Self
    }

    fn from_normalized_f32(value: f32) -> Self {
        value as Self
    }
}
