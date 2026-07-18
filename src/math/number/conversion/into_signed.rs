/// Convert a number into a signed type
pub const trait IntoSigned<Signed> {
    /// Convert the number into a signed type
    fn into_signed(self) -> Signed;
}

// Unsigned integers

const impl IntoSigned<u8> for u8 {
    fn into_signed(self) -> u8 {
        self
    }
}

const impl IntoSigned<i8> for u8 {
    fn into_signed(self) -> i8 {
        self as i8
    }
}

const impl IntoSigned<u16> for u16 {
    fn into_signed(self) -> u16 {
        self
    }
}

const impl IntoSigned<i16> for u16 {
    fn into_signed(self) -> i16 {
        self as i16
    }
}

const impl IntoSigned<u32> for u32 {
    fn into_signed(self) -> u32 {
        self
    }
}

const impl IntoSigned<i32> for u32 {
    fn into_signed(self) -> i32 {
        self as i32
    }
}

const impl IntoSigned<u64> for u64 {
    fn into_signed(self) -> u64 {
        self
    }
}

const impl IntoSigned<i64> for u64 {
    fn into_signed(self) -> i64 {
        self as i64
    }
}

const impl IntoSigned<u128> for u128 {
    fn into_signed(self) -> u128 {
        self
    }
}

const impl IntoSigned<i128> for u128 {
    fn into_signed(self) -> i128 {
        self as i128
    }
}

const impl IntoSigned<usize> for usize {
    fn into_signed(self) -> usize {
        self
    }
}

const impl IntoSigned<isize> for usize {
    fn into_signed(self) -> isize {
        self as isize
    }
}

// Signed integers

const impl IntoSigned<u8> for i8 {
    fn into_signed(self) -> u8 {
        self as u8
    }
}

const impl IntoSigned<i8> for i8 {
    fn into_signed(self) -> i8 {
        self
    }
}

const impl IntoSigned<u16> for i16 {
    fn into_signed(self) -> u16 {
        self as u16
    }
}

const impl IntoSigned<i16> for i16 {
    fn into_signed(self) -> i16 {
        self
    }
}

const impl IntoSigned<u32> for i32 {
    fn into_signed(self) -> u32 {
        self as u32
    }
}

const impl IntoSigned<i32> for i32 {
    fn into_signed(self) -> i32 {
        self
    }
}

const impl IntoSigned<u64> for i64 {
    fn into_signed(self) -> u64 {
        self as u64
    }
}

const impl IntoSigned<i64> for i64 {
    fn into_signed(self) -> i64 {
        self
    }
}

const impl IntoSigned<u128> for i128 {
    fn into_signed(self) -> u128 {
        self as u128
    }
}

const impl IntoSigned<i128> for i128 {
    fn into_signed(self) -> i128 {
        self
    }
}

const impl IntoSigned<usize> for isize {
    fn into_signed(self) -> usize {
        self as usize
    }
}

const impl IntoSigned<isize> for isize {
    fn into_signed(self) -> isize {
        self
    }
}

// Floating point numbers

const impl IntoSigned<f32> for f32 {
    fn into_signed(self) -> f32 {
        self
    }
}

const impl IntoSigned<f64> for f64 {
    fn into_signed(self) -> f64 {
        self
    }
}
