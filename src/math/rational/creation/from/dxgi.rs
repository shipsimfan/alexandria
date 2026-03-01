use crate::math::Rational;
use std::num::NonZeroU32;
use win32::dxgi::DXGI_RATIONAL;

impl Rational {
    /// Convert a [`DXGI_RATIONAL`] into a [`Rational`]
    #[allow(unused)]
    pub(crate) fn from_dxgi(dxgi: &DXGI_RATIONAL) -> Rational {
        Rational {
            numerator: dxgi.numerator as _,
            denominator: NonZeroU32::new(dxgi.denominator).unwrap(),
        }
    }
}
