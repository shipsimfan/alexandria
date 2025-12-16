///  A value that can be lossily converted into an [`f32`]
pub const trait IntoF32 {
    /// Converts `self` into an [`f32`]
    fn into_f32(self) -> f32;
}

// Unsigned integers

impl const IntoF32 for u8 {
    fn into_f32(self) -> f32 {
        self as f32
    }
}

impl const IntoF32 for u16 {
    fn into_f32(self) -> f32 {
        self as f32
    }
}

impl const IntoF32 for u32 {
    fn into_f32(self) -> f32 {
        self as f32
    }
}

impl const IntoF32 for u64 {
    fn into_f32(self) -> f32 {
        self as f32
    }
}

impl const IntoF32 for u128 {
    fn into_f32(self) -> f32 {
        self as f32
    }
}

impl const IntoF32 for usize {
    fn into_f32(self) -> f32 {
        self as f32
    }
}

// Signed integers

impl const IntoF32 for i8 {
    fn into_f32(self) -> f32 {
        self as f32
    }
}

impl const IntoF32 for i16 {
    fn into_f32(self) -> f32 {
        self as f32
    }
}

impl const IntoF32 for i32 {
    fn into_f32(self) -> f32 {
        self as f32
    }
}

impl const IntoF32 for i64 {
    fn into_f32(self) -> f32 {
        self as f32
    }
}

impl const IntoF32 for i128 {
    fn into_f32(self) -> f32 {
        self as f32
    }
}

impl const IntoF32 for isize {
    fn into_f32(self) -> f32 {
        self as f32
    }
}

// Floating-point

impl const IntoF32 for f32 {
    fn into_f32(self) -> f32 {
        self
    }
}

impl const IntoF32 for f64 {
    fn into_f32(self) -> f32 {
        self as f32
    }
}
