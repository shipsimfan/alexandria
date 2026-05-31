use crate::math::{
    Color3, ColorHsv, ColorSpace,
    number::{Floor, FromF32, Normalize, One, Zero},
};
use std::{
    marker::Destruct,
    ops::{Mul, Sub},
};

impl<T: Zero + One, Space: ColorSpace<T>> ColorHsv<T, Space> {
    /// Convert this [`ColorHsv`] into an RGB [`Color3`]
    pub const fn into_rgb(self) -> Color3<T, Space>
    where
        T: [const] Sub<T, Output = T>
            + [const] Mul<T, Output = T>
            + [const] Normalize
            + [const] Floor
            + [const] PartialEq
            + [const] Clone
            + [const] FromF32
            + [const] Destruct,
    {
        if self.s == T::ZERO {
            return Color3::new(self.v.clone(), self.v.clone(), self.v);
        }

        let h = self.h.normalize() * T::from_f32(6.0);
        let i = h.clone().floor();

        let f = h - i.clone();
        let p = self.v.clone() * (T::NORMALIZED_ONE - self.s.clone());
        let q = self.v.clone() * (T::NORMALIZED_ONE - f.clone() * self.s.clone());
        let t = self.v.clone() * (T::NORMALIZED_ONE - (T::NORMALIZED_ONE - f) * self.s);

        if i == T::ZERO {
            Color3::new(self.v, t, p)
        } else if i == T::ONE {
            Color3::new(q, self.v, p)
        } else if i == T::from_f32(2.0) {
            Color3::new(p, self.v, t)
        } else if i == T::from_f32(3.0) {
            Color3::new(p, q, self.v)
        } else if i == T::from_f32(4.0) {
            Color3::new(t, p, self.v)
        } else {
            Color3::new(self.v, p, q)
        }
    }
}

impl<
    T: Zero
        + One
        + [const] Sub<T, Output = T>
        + [const] Mul<T, Output = T>
        + [const] Normalize
        + [const] Floor
        + [const] PartialEq
        + [const] Clone
        + [const] FromF32
        + [const] Destruct,
    Space: ColorSpace<T>,
> const Into<Color3<T, Space>> for ColorHsv<T, Space>
{
    fn into(self) -> Color3<T, Space> {
        self.into_rgb()
    }
}

#[cfg(test)]
mod tests {
    use crate::math::color::{Color3, ColorHsv, Linear};

    macro_rules! hsv_to_rgb_tests {
        [
            $type: ty = $epsilon: literal:
            $(
                $test_name: ident: ($h: literal, $s: literal, $v: literal) -> ($r: literal, $g: literal, $b: literal),
            )*
        ] => {$(
            #[test]
            fn $test_name() {
                const EPSILON: $type = $epsilon;

                const INPUT: ColorHsv<$type, Linear> = ColorHsv::new($h, $s, $v);
                const OUTPUT: Color3<$type, Linear> = Color3::new($r, $g, $b);

                let output = INPUT.into_rgb();

                assert!(output.approx_eq(OUTPUT, EPSILON), "hsv to rgb failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    hsv_to_rgb_tests![
        f32 = 1e-5:
        hsv_to_rgb_red: (0.0, 1.0, 1.0) -> (1.0, 0.0, 0.0),
        hsv_to_rgb_green: (0.333333, 1.0, 1.0) -> (0.0, 1.0, 0.0),
        hsv_to_rgb_blue: (0.666667, 1.0, 1.0) -> (0.0, 0.0, 1.0),
        hsv_to_rgb_yellow: (0.166667, 1.0, 1.0) -> (1.0, 1.0, 0.0),
        hsv_to_rgb_cyan: (0.5, 1.0, 1.0) -> (0.0, 1.0, 1.0),
        hsv_to_rgb_magenta: (0.833333, 1.0, 1.0) -> (1.0, 0.0, 1.0),
        hsv_to_rgb_white: (0.0, 0.0, 1.0) -> (1.0, 1.0, 1.0),
        hsv_to_rgb_black: (0.0, 0.0, 0.0) -> (0.0, 0.0, 0.0),
        hsv_to_rgb_gray: (0.0, 0.0, 0.5) -> (0.5, 0.5, 0.5),
        hsv_to_rgb_dark_red: (0.0, 1.0, 0.5) -> (0.5, 0.0, 0.0),
        hsv_to_rgb_orange: (0.083333, 1.0, 1.0) -> (1.0, 0.5, 0.0),
        hsv_to_rgb_pink: (0.0, 0.5, 1.0) -> (1.0, 0.5, 0.5),
        hsv_to_rgb_light_green: (0.333333, 0.5, 1.0) -> (0.5, 1.0, 0.5),
        hsv_to_rgb_light_blue: (0.666667, 0.5, 1.0) -> (0.5, 0.5, 1.0),
        hsv_to_rgb_dark_green: (0.333333, 1.0, 0.5) -> (0.0, 0.5, 0.0),
        hsv_to_rgb_dark_blue: (0.666667, 1.0, 0.5) -> (0.0, 0.0, 0.5),
        hsv_to_rgb_purple: (0.75, 1.0, 1.0) -> (0.5, 0.0, 1.0),
        hsv_to_rgb_teal: (0.5, 1.0, 0.5) -> (0.0, 0.5, 0.5),
        hsv_to_rgb_olive: (0.166667, 1.0, 0.5) -> (0.5, 0.5, 0.0),
        hsv_to_rgb_violet: (0.833333, 1.0, 0.5) -> (0.5, 0.0, 0.5),
        hsv_to_rgb_lime: (0.25, 1.0, 1.0) -> (0.5, 1.0, 0.0),
        hsv_to_rgb_coral: (0.047619, 0.7, 1.0) -> (1.0, 0.5, 0.3),
        hsv_to_rgb_brown: (0.083333, 1.0, 0.6) -> (0.6, 0.3, 0.0),
        hsv_to_rgb_salmon: (0.0, 0.4, 1.0) -> (1.0, 0.6, 0.6),
        hsv_to_rgb_medium_sat_red: (0.0, 0.666667, 0.75) -> (0.75, 0.25, 0.25),
        hsv_to_rgb_medium_sat_green: (0.333333, 0.666667, 0.75) -> (0.25, 0.75, 0.25),
        hsv_to_rgb_medium_sat_blue: (0.666667, 0.666667, 0.75) -> (0.25, 0.25, 0.75),
        hsv_to_rgb_light_gray: (0.0, 0.0, 0.9) -> (0.9, 0.9, 0.9),
        hsv_to_rgb_dark_gray: (0.0, 0.0, 0.1) -> (0.1, 0.1, 0.1),
        hsv_to_rgb_low_sat_red: (0.0, 0.166667, 0.6) -> (0.6, 0.5, 0.5),
        hsv_to_rgb_low_sat_green: (0.333333, 0.166667, 0.6) -> (0.5, 0.6, 0.5),
    ];
}
