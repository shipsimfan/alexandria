use crate::math::Vector2;

mod cmp;
mod creation;
mod debug;
mod get;
mod interpolation;
mod into;
mod map;
mod math;
mod rounding;

/// The description of a rectangular area

#[cfg_attr(
    feature = "data-format",
    derive(data_format::Deserialize, data_format::Serialize)
)]
pub struct Rect<T> {
    /// The position of the top-left point of the rectangle
    pub position: Vector2<T>,

    /// The size of the rectangle
    pub size: Vector2<T>,
}

/// A [`Rect`] made up of [`f32`]s
pub type Rectf = Rect<f32>;

/// A [`Rect`] made up of [`f64`]s
pub type Rectd = Rect<f64>;

/// A [`Rect`] made up of [`u32`]s
pub type Rectu = Rect<u32>;

/// A [`Rect`] made up of [`i32`]s
pub type Recti = Rect<i32>;
