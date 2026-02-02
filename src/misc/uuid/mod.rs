mod as_bytes;
mod display;
mod from_str;
mod new;

#[cfg(feature = "data-format")]
mod data_format;

/// A universally unique identifier
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Uuid {
    /// The bytes that make up this UUID
    bytes: [u8; 16],
}
