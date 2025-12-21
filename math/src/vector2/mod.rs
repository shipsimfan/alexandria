mod creation;

/// A 2-dimensional vector
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "data-format",
    derive(data_format::Serialize, data_format::Deserialize)
)]
pub struct Vector2<T> {
    /// The x-axis value
    pub x: T,

    /// The y-axis value
    pub y: T,
}
