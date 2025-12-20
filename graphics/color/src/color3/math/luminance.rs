use crate::{Color3, Linear};
use alexandria_math::{Vector3, number::IntoF32};
use std::marker::Destruct;

impl<T> Color3<T, Linear> {
    /// Calculate the luminance of a [`Color3`] using coefficients `c`
    pub const fn luminance(self, c: Vector3<f32>) -> f32
    where
        T: [const] Destruct + [const] IntoF32,
    {
        self.r.into_normalized_f32() * c.x
            + self.g.into_normalized_f32() * c.y
            + self.b.into_normalized_f32() * c.z
    }
}

#[cfg(test)]
mod tests {
    use crate::{Color3, Linear};
    use alexandria_math::{Vector3, number::ApproxEq};

    macro_rules! luminance_tests {
        [
            $type: ty:
            $(
                $test_name: ident:
                    ($ir: literal, $ig: literal, $ib: literal),
                    ($cr: literal, $cg: literal, $cb: literal)
                    -> $luminance: literal,
            )*
        ] => {$(
            #[test]
            fn $test_name() {

                const INPUT: Color3<$type, Linear> = Color3::new($ir, $ig, $ib);
                const COEFFICIENTS: Vector3<f32> = Vector3::new($cr, $cg, $cb);
                const OUTPUT: f32 = $luminance;

                let output = INPUT.luminance(COEFFICIENTS);

                assert!(output.approx_eq(OUTPUT, 1e-6), "luminance failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    luminance_tests![
        f32:
        luminance_rec709_white: (1.0, 1.0, 1.0), (0.2126, 0.7152, 0.0722) -> 1.0,
        luminance_rec709_black: (0.0, 0.0, 0.0), (0.2126, 0.7152, 0.0722) -> 0.0,
        luminance_rec709_red: (1.0, 0.0, 0.0), (0.2126, 0.7152, 0.0722) -> 0.2126,
        luminance_rec709_green: (0.0, 1.0, 0.0), (0.2126, 0.7152, 0.0722) -> 0.7152,
        luminance_rec709_blue: (0.0, 0.0, 1.0), (0.2126, 0.7152, 0.0722) -> 0.0722,
        luminance_rec709_random1: (0.2, 0.4, 0.6), (0.2126, 0.7152, 0.0722) -> 0.37192,
        luminance_rec709_random2: (0.9, 0.1, 0.3), (0.2126, 0.7152, 0.0722) -> 0.28452,

        luminance_rec601_white: (1.0, 1.0, 1.0), (0.299, 0.587, 0.114) -> 1.0,
        luminance_rec601_black: (0.0, 0.0, 0.0), (0.299, 0.587, 0.114) -> 0.0,
        luminance_rec601_red: (1.0, 0.0, 0.0), (0.299, 0.587, 0.114) -> 0.299,
        luminance_rec601_green: (0.0, 1.0, 0.0), (0.299, 0.587, 0.114) -> 0.587,
        luminance_rec601_blue: (0.0, 0.0, 1.0), (0.299, 0.587, 0.114) -> 0.114,
        luminance_rec601_random1: (0.2, 0.4, 0.6), (0.299, 0.587, 0.114) -> 0.363,
        luminance_rec601_random2: (0.9, 0.1, 0.3), (0.299, 0.587, 0.114) -> 0.362,

        luminance_rec2020_white: (1.0, 1.0, 1.0), (0.2627, 0.678, 0.0593) -> 1.0,
        luminance_rec2020_black: (0.0, 0.0, 0.0), (0.2627, 0.678, 0.0593) -> 0.0,
        luminance_rec2020_red: (1.0, 0.0, 0.0), (0.2627, 0.678, 0.0593) -> 0.2627,
        luminance_rec2020_green: (0.0, 1.0, 0.0), (0.2627, 0.678, 0.0593) -> 0.678,
        luminance_rec2020_blue: (0.0, 0.0, 1.0), (0.2627, 0.678, 0.0593) -> 0.0593,
        luminance_rec2020_random1: (0.2, 0.4, 0.6), (0.2627, 0.678, 0.0593) -> 0.35932,
        luminance_rec2020_random2: (0.9, 0.1, 0.3), (0.2627, 0.678, 0.0593) -> 0.32202,

        luminance_equal_white: (1.0, 1.0, 1.0), (0.3333333, 0.3333333, 0.3333333) -> 1.0,
        luminance_equal_black: (0.0, 0.0, 0.0), (0.3333333, 0.3333333, 0.3333333) -> 0.0,
        luminance_equal_red: (1.0, 0.0, 0.0), (0.3333333, 0.3333333, 0.3333333) -> 0.3333333,
        luminance_equal_green: (0.0, 1.0, 0.0), (0.3333333, 0.3333333, 0.3333333) -> 0.3333333,
        luminance_equal_blue: (0.0, 0.0, 1.0), (0.3333333, 0.3333333, 0.3333333) -> 0.3333333,
        luminance_equal_random1: (0.2, 0.4, 0.6), (0.3333333, 0.3333333, 0.3333333) -> 0.4,
        luminance_equal_random2: (0.9, 0.1, 0.3), (0.3333333, 0.3333333, 0.3333333) -> 0.433333,
    ];

    luminance_tests![
        u8:
        luminance_rec709_white_u8: (255, 255, 255), (0.2126, 0.7152, 0.0722) -> 1.0,
        luminance_rec709_black_u8: (0, 0, 0), (0.2126, 0.7152, 0.0722) -> 0.0,
        luminance_rec709_red_u8: (255, 0, 0), (0.2126, 0.7152, 0.0722) -> 0.2126,
        luminance_rec709_green_u8: (0, 255, 0), (0.2126, 0.7152, 0.0722) -> 0.7152,
        luminance_rec709_blue_u8: (0, 0, 255), (0.2126, 0.7152, 0.0722) -> 0.0722,

        luminance_rec601_white_u8: (255, 255, 255), (0.299, 0.587, 0.114) -> 1.0,
        luminance_rec601_black_u8: (0, 0, 0), (0.299, 0.587, 0.114) -> 0.0,
        luminance_rec601_red_u8: (255, 0, 0), (0.299, 0.587, 0.114) -> 0.299,
        luminance_rec601_green_u8: (0, 255, 0), (0.299, 0.587, 0.114) -> 0.587,
        luminance_rec601_blue_u8: (0, 0, 255), (0.299, 0.587, 0.114) -> 0.114,

        luminance_rec2020_white_u8: (255, 255, 255), (0.2627, 0.678, 0.0593) -> 1.0,
        luminance_rec2020_black_u8: (0, 0, 0), (0.2627, 0.678, 0.0593) -> 0.0,
        luminance_rec2020_red_u8: (255, 0, 0), (0.2627, 0.678, 0.0593) -> 0.2627,
        luminance_rec2020_green_u8: (0, 255, 0), (0.2627, 0.678, 0.0593) -> 0.678,
        luminance_rec2020_blue_u8: (0, 0, 255), (0.2627, 0.678, 0.0593) -> 0.0593,
    ];
}
