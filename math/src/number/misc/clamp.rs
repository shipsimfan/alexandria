/// Restrict a value to a certain interval
pub const trait Clamp: Sized {
    /// The bound to use for `min` and `max`
    type Bound = Self;

    /// Restrict a value to a certain interval
    fn clamp(self, min: Self::Bound, max: Self::Bound) -> Self;
}

// Unsigned integers

impl const Clamp for u8 {
    fn clamp(self, min: Self, max: Self) -> Self {
        Ord::clamp(self, min, max)
    }
}

impl const Clamp for u16 {
    fn clamp(self, min: Self, max: Self) -> Self {
        Ord::clamp(self, min, max)
    }
}

impl const Clamp for u32 {
    fn clamp(self, min: Self, max: Self) -> Self {
        Ord::clamp(self, min, max)
    }
}

impl const Clamp for u64 {
    fn clamp(self, min: Self, max: Self) -> Self {
        Ord::clamp(self, min, max)
    }
}

impl const Clamp for u128 {
    fn clamp(self, min: Self, max: Self) -> Self {
        Ord::clamp(self, min, max)
    }
}

impl const Clamp for usize {
    fn clamp(self, min: Self, max: Self) -> Self {
        Ord::clamp(self, min, max)
    }
}

// Signed integers

impl const Clamp for i8 {
    fn clamp(self, min: Self, max: Self) -> Self {
        Ord::clamp(self, min, max)
    }
}

impl const Clamp for i16 {
    fn clamp(self, min: Self, max: Self) -> Self {
        Ord::clamp(self, min, max)
    }
}

impl const Clamp for i32 {
    fn clamp(self, min: Self, max: Self) -> Self {
        Ord::clamp(self, min, max)
    }
}

impl const Clamp for i64 {
    fn clamp(self, min: Self, max: Self) -> Self {
        Ord::clamp(self, min, max)
    }
}

impl const Clamp for i128 {
    fn clamp(self, min: Self, max: Self) -> Self {
        Ord::clamp(self, min, max)
    }
}

impl const Clamp for isize {
    fn clamp(self, min: Self, max: Self) -> Self {
        Ord::clamp(self, min, max)
    }
}

// Floating-point
impl const Clamp for f32 {
    fn clamp(self, min: Self, max: Self) -> Self {
        f32::clamp(self, min, max)
    }
}

impl const Clamp for f64 {
    fn clamp(self, min: Self, max: Self) -> Self {
        f64::clamp(self, min, max)
    }
}
