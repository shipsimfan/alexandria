/// Convert a signed type into an unsigned type
pub const trait FromSigned<Signed> {
    /// Convert the number into an unsigned type
    fn from_signed(signed: Signed) -> Self;
}

// Unsigned integers

impl const FromSigned<u8> for u8 {
    fn from_signed(signed: u8) -> Self {
        signed
    }
}

impl const FromSigned<i8> for u8 {
    fn from_signed(signed: i8) -> Self {
        signed as u8
    }
}

impl const FromSigned<u16> for u16 {
    fn from_signed(signed: u16) -> Self {
        signed
    }
}

impl const FromSigned<i16> for u16 {
    fn from_signed(signed: i16) -> Self {
        signed as u16
    }
}

impl const FromSigned<u32> for u32 {
    fn from_signed(signed: u32) -> Self {
        signed
    }
}

impl const FromSigned<i32> for u32 {
    fn from_signed(signed: i32) -> Self {
        signed as u32
    }
}

impl const FromSigned<u64> for u64 {
    fn from_signed(signed: u64) -> Self {
        signed
    }
}

impl const FromSigned<i64> for u64 {
    fn from_signed(signed: i64) -> Self {
        signed as u64
    }
}

impl const FromSigned<u128> for u128 {
    fn from_signed(signed: u128) -> Self {
        signed
    }
}

impl const FromSigned<i128> for u128 {
    fn from_signed(signed: i128) -> Self {
        signed as u128
    }
}

impl const FromSigned<usize> for usize {
    fn from_signed(signed: usize) -> Self {
        signed
    }
}

impl const FromSigned<isize> for usize {
    fn from_signed(signed: isize) -> Self {
        signed as usize
    }
}

// Signed integers

impl const FromSigned<u8> for i8 {
    fn from_signed(signed: u8) -> Self {
        signed as i8
    }
}

impl const FromSigned<i8> for i8 {
    fn from_signed(signed: i8) -> Self {
        signed
    }
}

impl const FromSigned<u16> for i16 {
    fn from_signed(signed: u16) -> Self {
        signed as i16
    }
}

impl const FromSigned<i16> for i16 {
    fn from_signed(signed: i16) -> Self {
        signed
    }
}

impl const FromSigned<u32> for i32 {
    fn from_signed(signed: u32) -> Self {
        signed as i32
    }
}

impl const FromSigned<i32> for i32 {
    fn from_signed(signed: i32) -> Self {
        signed
    }
}

impl const FromSigned<u64> for i64 {
    fn from_signed(signed: u64) -> Self {
        signed as i64
    }
}

impl const FromSigned<i64> for i64 {
    fn from_signed(signed: i64) -> Self {
        signed
    }
}

impl const FromSigned<u128> for i128 {
    fn from_signed(signed: u128) -> Self {
        signed as i128
    }
}

impl const FromSigned<i128> for i128 {
    fn from_signed(signed: i128) -> Self {
        signed
    }
}

impl const FromSigned<usize> for isize {
    fn from_signed(signed: usize) -> Self {
        signed as isize
    }
}

impl const FromSigned<isize> for isize {
    fn from_signed(signed: isize) -> Self {
        signed
    }
}

// Floating point numbers

impl const FromSigned<f32> for f32 {
    fn from_signed(signed: f32) -> Self {
        signed
    }
}

impl const FromSigned<f64> for f64 {
    fn from_signed(signed: f64) -> Self {
        signed
    }
}
