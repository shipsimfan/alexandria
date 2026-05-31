use crate::math::{
    Color3, ColorHsv, ColorSpace,
    number::{FromF32, Max, Min, One, Zero},
};
use std::{
    marker::Destruct,
    ops::{Add, AddAssign, Div, Sub},
};

impl<T, Space: ColorSpace<T>> ColorHsv<T, Space> {
    /// Create a new [`ColorHsv`] from an RGB [`Color3`]
    pub const fn from_rgb(rgb: Color3<T, Space>) -> ColorHsv<T, Space>
    where
        T: Zero
            + One
            + [const] Add<T, Output = T>
            + [const] AddAssign<T>
            + [const] Sub<T, Output = T>
            + [const] Div<T, Output = T>
            + [const] Max
            + [const] Min
            + [const] PartialEq
            + [const] PartialOrd
            + [const] Clone
            + [const] Destruct
            + [const] FromF32,
    {
        let max_c = rgb.r.clone().max(rgb.g.clone()).max(rgb.b.clone());
        let min_c = rgb.r.clone().min(rgb.g.clone()).min(rgb.b.clone());
        let delta = max_c.clone() - min_c;

        if max_c == T::ZERO {
            return ColorHsv::BLACK;
        }

        let mut h = if delta == T::ZERO {
            delta.clone()
        } else if max_c == rgb.r {
            (rgb.g - rgb.b) / delta.clone()
        } else if max_c == rgb.g {
            (rgb.b - rgb.r) / delta.clone() + T::from_f32(2.0)
        } else {
            (rgb.r - rgb.g) / delta.clone() + T::from_f32(4.0)
        } / T::from_f32(6.0);

        if h < T::ZERO {
            h += T::NORMALIZED_ONE;
        }

        ColorHsv::new(h, delta / max_c.clone(), max_c)
    }
}

impl<
    T: Zero
        + One
        + [const] Add<T, Output = T>
        + [const] AddAssign<T>
        + [const] Sub<T, Output = T>
        + [const] Div<T, Output = T>
        + [const] Max
        + [const] Min
        + [const] PartialEq
        + [const] PartialOrd
        + [const] Clone
        + [const] Destruct
        + [const] FromF32,
    Space: ColorSpace<T>,
> const From<Color3<T, Space>> for ColorHsv<T, Space>
{
    fn from(value: Color3<T, Space>) -> Self {
        ColorHsv::from_rgb(value)
    }
}

#[cfg(test)]
mod tests {
    use crate::math::color::{Color3, ColorHsv, Linear};

    macro_rules! rgb_to_hsv_tests {
        [
            $type: ty = $epsilon: literal:
            $(
                $test_name: ident: ($r: literal, $g: literal, $b: literal) -> ($h: literal, $s: literal, $v: literal),
            )*
        ] => {$(
            #[test]
            fn $test_name() {
                const EPSILON: $type = $epsilon;

                const INPUT: Color3<$type, Linear> = Color3::new($r, $g, $b);
                const OUTPUT: ColorHsv<$type, Linear> = ColorHsv::new($h, $s, $v);

                let output = ColorHsv::from_rgb(INPUT);

                assert!(output.approx_eq(OUTPUT, EPSILON), "rgb to hsv failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    rgb_to_hsv_tests![
        f32 = 1e-6:
        rgb_to_hsv_red: (1.0, 0.0, 0.0) -> (0.0, 1.0, 1.0),
        rgb_to_hsv_green: (0.0, 1.0, 0.0) -> (0.333333, 1.0, 1.0),
        rgb_to_hsv_blue: (0.0, 0.0, 1.0) -> (0.666667, 1.0, 1.0),
        rgb_to_hsv_yellow: (1.0, 1.0, 0.0) -> (0.166667, 1.0, 1.0),
        rgb_to_hsv_cyan: (0.0, 1.0, 1.0) -> (0.5, 1.0, 1.0),
        rgb_to_hsv_magenta: (1.0, 0.0, 1.0) -> (0.833333, 1.0, 1.0),
        rgb_to_hsv_white: (1.0, 1.0, 1.0) -> (0.0, 0.0, 1.0),
        rgb_to_hsv_black: (0.0, 0.0, 0.0) -> (0.0, 0.0, 0.0),
        rgb_to_hsv_gray: (0.5, 0.5, 0.5) -> (0.0, 0.0, 0.5),
        rgb_to_hsv_dark_red: (0.5, 0.0, 0.0) -> (0.0, 1.0, 0.5),
        rgb_to_hsv_orange: (1.0, 0.5, 0.0) -> (0.083333, 1.0, 1.0),
        rgb_to_hsv_pink: (1.0, 0.5, 0.5) -> (0.0, 0.5, 1.0),
        rgb_to_hsv_light_green: (0.5, 1.0, 0.5) -> (0.333333, 0.5, 1.0),
        rgb_to_hsv_light_blue: (0.5, 0.5, 1.0) -> (0.666667, 0.5, 1.0),
        rgb_to_hsv_dark_green: (0.0, 0.5, 0.0) -> (0.333333, 1.0, 0.5),
        rgb_to_hsv_dark_blue: (0.0, 0.0, 0.5) -> (0.666667, 1.0, 0.5),
        rgb_to_hsv_purple: (0.5, 0.0, 1.0) -> (0.75, 1.0, 1.0),
        rgb_to_hsv_teal: (0.0, 0.5, 0.5) -> (0.5, 1.0, 0.5),
        rgb_to_hsv_olive: (0.5, 0.5, 0.0) -> (0.166667, 1.0, 0.5),
        rgb_to_hsv_violet: (0.5, 0.0, 0.5) -> (0.833333, 1.0, 0.5),
        rgb_to_hsv_lime: (0.5, 1.0, 0.0) -> (0.25, 1.0, 1.0),
        rgb_to_hsv_coral: (1.0, 0.5, 0.3) -> (0.047619, 0.7, 1.0),
        rgb_to_hsv_brown: (0.6, 0.3, 0.0) -> (0.083333, 1.0, 0.6),
        rgb_to_hsv_salmon: (1.0, 0.6, 0.6) -> (0.0, 0.4, 1.0),
        rgb_to_hsv_medium_sat_red: (0.75, 0.25, 0.25) -> (0.0, 0.666667, 0.75),
        rgb_to_hsv_medium_sat_green: (0.25, 0.75, 0.25) -> (0.333333, 0.666667, 0.75),
        rgb_to_hsv_medium_sat_blue: (0.25, 0.25, 0.75) -> (0.666667, 0.666667, 0.75),
        rgb_to_hsv_light_gray: (0.9, 0.9, 0.9) -> (0.0, 0.0, 0.9),
        rgb_to_hsv_dark_gray: (0.1, 0.1, 0.1) -> (0.0, 0.0, 0.1),
        rgb_to_hsv_low_sat_red: (0.6, 0.5, 0.5) -> (0.0, 0.166667, 0.6),
        rgb_to_hsv_low_sat_green: (0.5, 0.6, 0.5) -> (0.333333, 0.166667, 0.6),
    ];
}
