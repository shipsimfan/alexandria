//! Traits for defining what numeric operations types can perform

mod absolute;
mod ceil;
mod floor;
mod fract;
mod infinity;
mod into_f32;
mod max;
mod min;
mod nan;
mod neg_infinity;
mod one;
mod round;
mod sqrt;
mod zero;

pub use absolute::Absolute;
pub use ceil::Ceil;
pub use floor::Floor;
pub use fract::Fract;
pub use infinity::Infinity;
pub use into_f32::IntoF32;
pub use max::Max;
pub use min::Min;
pub use nan::NaN;
pub use neg_infinity::NegInfinity;
pub use one::One;
pub use round::Round;
pub use sqrt::Sqrt;
pub use zero::Zero;
