use crate::Uuid;
use std::str::FromStr;

/// An invalid UUID was parsed
#[derive(Debug)]
pub struct InvalidUUID;

impl std::error::Error for InvalidUUID {}

impl std::fmt::Display for InvalidUUID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        "invalid UUID".fmt(f)
    }
}

/// Convert `b` from an ASCII hex digit
const fn to_ascii_hexdigit(b: u8) -> Result<u8, InvalidUUID> {
    if b >= b'0' && b <= b'9' {
        Ok(b - b'0')
    } else if b >= b'a' && b <= b'f' {
        Ok(b - b'a' + 10)
    } else if b >= b'A' && b <= b'F' {
        Ok(b - b'A' + 10)
    } else {
        Err(InvalidUUID)
    }
}

impl Uuid {
    /// Attempt to parse `s` into a [`UUID`]
    pub const fn from_str(s: &str) -> Result<Self, InvalidUUID> {
        // Validate length
        if s.len() != 36 {
            return Err(InvalidUUID);
        }

        // Parse bytes
        let mut bytes = [0; 16];
        let mut si = 0;
        let mut bi = 0;
        while si < 36 {
            // Check for dashes
            if si == 8 || si == 13 || si == 18 || si == 23 {
                if s.as_bytes()[si] != b'-' {
                    return Err(InvalidUUID);
                }

                si += 1;
                continue;
            }

            // Parse hexdigit
            let high = s.as_bytes()[si];
            let low = s.as_bytes()[si + 1];
            si += 2;

            let high = to_ascii_hexdigit(high)?;
            let low = to_ascii_hexdigit(low)?;

            bytes[bi] = high << 4 | low;
            bi += 1;
        }

        Ok(Uuid::from_flat(bytes))
    }
}

impl const FromStr for Uuid {
    type Err = InvalidUUID;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str(s)
    }
}
