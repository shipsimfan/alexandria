mod to_d3d;

/// The type an input element is
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputElementType {
    /// The input element is a 32-bit unsigned integer
    U32,

    /// The input element is a 32-bit floating point number
    F32,

    /// The input element is a 2-length vector of 32-bit floating point numbers
    Vector2F32,

    /// The input element is a 3-length vector of 32-bit floating point numbers
    Vector3F32,

    /// The input element is a 4-length vector of 32-bit floating point numbers
    Vector4F32,
}
