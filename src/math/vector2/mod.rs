mod abs;
mod add;
mod ceil;
mod clamp;
mod constants;
mod display;
mod div;
mod dot;
mod floor;
mod fract;
mod from;
mod index;
mod into;
mod length;
mod max;
mod min;
mod mul;
mod neg;
mod new;
mod normalize;
mod rem;
mod round;
mod saturate;
mod sqrt;
mod sub;
mod swizzle;

/// A vector of size 2
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vector2<T> {
    /// The x-axis value of the vector
    pub x: T,

    /// The y-axis value of the vector
    pub y: T,
}

/// A vector of size 2 made of [`f32`]s
pub type Vector2f = Vector2<f32>;

/// A vector of size 2 made of [`f64`]s
pub type Vector2d = Vector2<f64>;

/// A vector of size 2 made of [`u32`]s
pub type Vector2u = Vector2<u32>;

/// A vector of size 2 made of [`i32`]s
pub type Vector2i = Vector2<i32>;
