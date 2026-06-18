/// Convert a number into a signed type
pub const trait IntoSigned<Signed> {
    /// Convert the number into a signed type
    fn into_signed(self) -> Signed;
}

// Unsigned integers

impl const IntoSigned<u8> for u8 {
    fn into_signed(self) -> u8 {
        self
    }
}

impl const IntoSigned<i8> for u8 {
    fn into_signed(self) -> i8 {
        self as i8
    }
}

impl const IntoSigned<u16> for u16 {
    fn into_signed(self) -> u16 {
        self
    }
}

impl const IntoSigned<i16> for u16 {
    fn into_signed(self) -> i16 {
        self as i16
    }
}

impl const IntoSigned<u32> for u32 {
    fn into_signed(self) -> u32 {
        self
    }
}

impl const IntoSigned<i32> for u32 {
    fn into_signed(self) -> i32 {
        self as i32
    }
}

impl const IntoSigned<u64> for u64 {
    fn into_signed(self) -> u64 {
        self
    }
}

impl const IntoSigned<i64> for u64 {
    fn into_signed(self) -> i64 {
        self as i64
    }
}

impl const IntoSigned<u128> for u128 {
    fn into_signed(self) -> u128 {
        self
    }
}

impl const IntoSigned<i128> for u128 {
    fn into_signed(self) -> i128 {
        self as i128
    }
}

impl const IntoSigned<usize> for usize {
    fn into_signed(self) -> usize {
        self
    }
}

impl const IntoSigned<isize> for usize {
    fn into_signed(self) -> isize {
        self as isize
    }
}

// Signed integers

impl const IntoSigned<u8> for i8 {
    fn into_signed(self) -> u8 {
        self as u8
    }
}

impl const IntoSigned<i8> for i8 {
    fn into_signed(self) -> i8 {
        self
    }
}

impl const IntoSigned<u16> for i16 {
    fn into_signed(self) -> u16 {
        self as u16
    }
}

impl const IntoSigned<i16> for i16 {
    fn into_signed(self) -> i16 {
        self
    }
}

impl const IntoSigned<u32> for i32 {
    fn into_signed(self) -> u32 {
        self as u32
    }
}

impl const IntoSigned<i32> for i32 {
    fn into_signed(self) -> i32 {
        self
    }
}

impl const IntoSigned<u64> for i64 {
    fn into_signed(self) -> u64 {
        self as u64
    }
}

impl const IntoSigned<i64> for i64 {
    fn into_signed(self) -> i64 {
        self
    }
}

impl const IntoSigned<u128> for i128 {
    fn into_signed(self) -> u128 {
        self as u128
    }
}

impl const IntoSigned<i128> for i128 {
    fn into_signed(self) -> i128 {
        self
    }
}

impl const IntoSigned<usize> for isize {
    fn into_signed(self) -> usize {
        self as usize
    }
}

impl const IntoSigned<isize> for isize {
    fn into_signed(self) -> isize {
        self
    }
}

// Floating point numbers

impl const IntoSigned<f32> for f32 {
    fn into_signed(self) -> f32 {
        self
    }
}

impl const IntoSigned<f64> for f64 {
    fn into_signed(self) -> f64 {
        self
    }
}
