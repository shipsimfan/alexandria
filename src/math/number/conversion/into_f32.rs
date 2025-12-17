///  A value that can be lossily converted into an [`f32`]
pub const trait IntoF32 {
    /// Converts `self` into an [`f32`]
    fn into_f32(self) -> f32;

    /// Converts `self` into an [`f32`] in the range [-1.0, 1.0], or [0.0, 1.0] for unsigned types
    fn into_normalized_f32(self) -> f32;
}

// Unsigned integers

impl const IntoF32 for u8 {
    fn into_f32(self) -> f32 {
        self as f32
    }

    fn into_normalized_f32(self) -> f32 {
        self as f32 / u8::MAX as f32
    }
}

impl const IntoF32 for u16 {
    fn into_f32(self) -> f32 {
        self as f32
    }

    fn into_normalized_f32(self) -> f32 {
        self as f32 / u16::MAX as f32
    }
}

impl const IntoF32 for u32 {
    fn into_f32(self) -> f32 {
        self as f32
    }

    fn into_normalized_f32(self) -> f32 {
        self as f32 / u32::MAX as f32
    }
}

impl const IntoF32 for u64 {
    fn into_f32(self) -> f32 {
        self as f32
    }

    fn into_normalized_f32(self) -> f32 {
        self as f32 / u64::MAX as f32
    }
}

impl const IntoF32 for u128 {
    fn into_f32(self) -> f32 {
        self as f32
    }

    fn into_normalized_f32(self) -> f32 {
        self as f32 / u128::MAX as f32
    }
}

impl const IntoF32 for usize {
    fn into_f32(self) -> f32 {
        self as f32
    }

    fn into_normalized_f32(self) -> f32 {
        self as f32 / usize::MAX as f32
    }
}

// Signed integers

impl const IntoF32 for i8 {
    fn into_f32(self) -> f32 {
        self as f32
    }

    fn into_normalized_f32(self) -> f32 {
        if self == i8::MIN {
            -1.0
        } else {
            self as f32 / i8::MAX as f32
        }
    }
}

impl const IntoF32 for i16 {
    fn into_f32(self) -> f32 {
        self as f32
    }

    fn into_normalized_f32(self) -> f32 {
        if self == i16::MIN {
            -1.0
        } else {
            self as f32 / i16::MAX as f32
        }
    }
}

impl const IntoF32 for i32 {
    fn into_f32(self) -> f32 {
        self as f32
    }

    fn into_normalized_f32(self) -> f32 {
        if self == i32::MIN {
            -1.0
        } else {
            self as f32 / i32::MAX as f32
        }
    }
}

impl const IntoF32 for i64 {
    fn into_f32(self) -> f32 {
        self as f32
    }

    fn into_normalized_f32(self) -> f32 {
        if self == i64::MIN {
            -1.0
        } else {
            self as f32 / i64::MAX as f32
        }
    }
}

impl const IntoF32 for i128 {
    fn into_f32(self) -> f32 {
        self as f32
    }

    fn into_normalized_f32(self) -> f32 {
        if self == i128::MIN {
            -1.0
        } else {
            self as f32 / i128::MAX as f32
        }
    }
}

impl const IntoF32 for isize {
    fn into_f32(self) -> f32 {
        self as f32
    }

    fn into_normalized_f32(self) -> f32 {
        if self == isize::MIN {
            -1.0
        } else {
            self as f32 / isize::MAX as f32
        }
    }
}

// Floating-point

impl const IntoF32 for f32 {
    fn into_f32(self) -> f32 {
        self
    }

    fn into_normalized_f32(self) -> f32 {
        self
    }
}

impl const IntoF32 for f64 {
    fn into_f32(self) -> f32 {
        self as f32
    }

    fn into_normalized_f32(self) -> f32 {
        self as f32
    }
}
