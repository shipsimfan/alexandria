use crate::math::{Color4, Linear, Vector3, number::IntoF32};
use std::marker::Destruct;

impl<T> Color4<T, Linear> {
    /// Calculate the luminance of a [`Color4`] using coefficients `c`
    pub const fn luminance(self, c: Vector3<f32>) -> f32
    where
        T: [const] Destruct + [const] IntoF32,
    {
        self.rgb().luminance(c)
    }
}

#[cfg(test)]
mod tests {
    use crate::math::{Color4, Linear, Vector3, number::ApproxEq};

    macro_rules! luminance_tests {
        [
            $type: ty:
            $(
                $test_name: ident:
                    ($ir: literal, $ig: literal, $ib: literal, $ia: literal),
                    ($cr: literal, $cg: literal, $cb: literal)
                    -> $luminance: literal,
            )*
        ] => {$(
            #[test]
            fn $test_name() {
                const INPUT: Color4<$type, Linear> = Color4::new($ir, $ig, $ib, $ia);
                const COEFFICIENTS: Vector3<f32> = Vector3::new($cr, $cg, $cb);
                const OUTPUT: f32 = $luminance;

                let output = INPUT.luminance(COEFFICIENTS);

                assert!(output.approx_eq(OUTPUT, 1e-6), "luminance failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    luminance_tests![
        f32:
        luminance_white_rec709: (1.0, 1.0, 1.0, 1.0), (0.2126, 0.7152, 0.0722) -> 1.0,
        luminance_white_rec601: (1.0, 1.0, 1.0, 1.0), (0.299, 0.587, 0.114) -> 1.0,
        luminance_red_rec709: (1.0, 0.0, 0.0, 1.0), (0.2126, 0.7152, 0.0722) -> 0.2126,
        luminance_green_rec709: (0.0, 1.0, 0.0, 1.0), (0.2126, 0.7152, 0.0722) -> 0.7152,
        luminance_blue_rec709: (0.0, 0.0, 1.0, 1.0), (0.2126, 0.7152, 0.0722) -> 0.0722,
        luminance_random1_rec709: (0.1, 0.5, 0.9, 0.3), (0.2126, 0.7152, 0.0722) -> 0.44384,
        luminance_random1_rec601: (0.1, 0.5, 0.9, 0.3), (0.299, 0.587, 0.114) -> 0.426,
        luminance_random2_rec2020: (0.9, 0.2, 0.4, 1.0), (0.2627, 0.678, 0.0593) -> 0.39575,
        luminance_gray25_equal: (0.25, 0.25, 0.25, 0.5), (0.33333334, 0.33333334, 0.33333334) -> 0.25,
        luminance_hdr_rec709: (1.5, 0.5, 0.0, 1.0), (0.2126, 0.7152, 0.0722) -> 0.6765,
        luminance_neg_rec601: (-0.2, 0.4, 0.6, 1.0), (0.299, 0.587, 0.114) -> 0.2434,
        luminance_black_rec2020: (0.0, 0.0, 0.0, 1.0), (0.2627, 0.678, 0.0593) -> 0.0,
    ];

    luminance_tests![
        u8:
        luminance_white_rec709_u8: (255, 255, 255, 255), (0.2126, 0.7152, 0.0722) -> 1.0,
        luminance_red_rec709_u8: (255, 0, 0, 255), (0.2126, 0.7152, 0.0722) -> 0.2126,
        luminance_random1_rec709_u8: (26, 128, 230, 77), (0.2126, 0.7152, 0.0722) -> 0.44580078,
        luminance_random1_rec601_u8: (26, 128, 230, 77), (0.299, 0.587, 0.114) -> 0.42796078,
        luminance_random2_rec2020_u8: (230, 51, 102, 255), (0.2627, 0.678, 0.0593) -> 0.3962651,
        luminance_gray25_equal_u8: (64, 64, 64, 128), (0.33333334, 0.33333334, 0.33333334) -> 0.2509804,
    ];
}
