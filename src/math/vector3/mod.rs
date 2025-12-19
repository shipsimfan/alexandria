mod creation;

/// A 3-dimensional vector
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "data-format", derive(data_format::Serialize))]
pub struct Vector3<T> {
    /// The x-axis value
    pub x: T,

    /// The y-axis value
    pub y: T,

    /// The z-axis value
    pub z: T,
}
