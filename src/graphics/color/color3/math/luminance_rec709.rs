use crate::{
    graphics::color::{Color3, Linear},
    math::{Vector3, number::IntoF32},
};
use std::marker::Destruct;

impl<T> Color3<T, Linear> {
    /// Calculate the Rec.709 luminance for this color
    pub const fn luminance_rec709(self) -> f32
    where
        T: [const] Destruct + [const] IntoF32,
    {
        self.luminance(Vector3::new(0.2126, 0.7152, 0.0722))
    }
}

macro_rules! luminance_tests {
    [
        $type: ty:
        $(
            $test_name: ident: ($ir: literal, $ig: literal, $ib: literal) -> $luminance: literal,
        )*
    ] => {$(
        #[test]
        fn $test_name() {
            use crate::math::number::ApproxEq;

            const INPUT: Color3<$type, crate::graphics::color::Linear> = Color3::new($ir, $ig, $ib);
            const OUTPUT: f32 = $luminance;

            let output = INPUT.luminance_rec709();

            assert!(output.approx_eq(OUTPUT, 1e-6), "luminance rec709 failed: {} vs. {}", output, OUTPUT);
        }
    )*};
}

luminance_tests![
    f32:
    luminance_rec709_black: (0.0, 0.0, 0.0) -> 0.0,
    luminance_rec709_white: (1.0, 1.0, 1.0) -> 1.0,

    luminance_rec709_red: (1.0, 0.0, 0.0) -> 0.2126,
    luminance_rec709_green: (0.0, 1.0, 0.0) -> 0.7152,
    luminance_rec709_blue: (0.0, 0.0, 1.0) -> 0.0722,

    luminance_rec709_mid_gray: (0.5, 0.5, 0.5) -> 0.5,

    luminance_rec709_hot_pinkish: (1.0, 0.2, 0.6) -> 0.39896,
    luminance_rec709_sky: (0.2, 0.4, 0.9) -> 0.39358,
    luminance_rec709_warm: (0.9, 0.6, 0.2) -> 0.6349,

    luminance_rec709_random1: (0.13, 0.77, 0.42) -> 0.608666,
    luminance_rec709_random2: (0.999, 0.001, 0.5) -> 0.2492026,

    luminance_rec709_negative: (-0.1, 0.2, 0.3) -> 0.14344,
    luminance_rec709_hdr_bright: (2.0, 1.5, 0.5) -> 1.5341,
];

luminance_tests![
    u8:
    luminance_rec709_u8_black: (0, 0, 0) -> 0.0,
    luminance_rec709_u8_white: (255, 255, 255) -> 1.0,

    luminance_rec709_u8_red: (255, 0, 0) -> 0.2126,
    luminance_rec709_u8_green: (0, 255, 0) -> 0.7152,
    luminance_rec709_u8_blue: (0, 0, 255) -> 0.0722,

    luminance_rec709_u8_mid_gray_128: (128, 128, 128) -> 0.5019608,

    luminance_rec709_u8_pinkish: (255, 51, 153) -> 0.39896,
    luminance_rec709_u8_skyish: (51, 102, 230) -> 0.3937216,
    luminance_rec709_u8_warmish: (230, 153, 51) -> 0.63531685,

    luminance_rec709_u8_random1: (33, 196, 107) -> 0.607531,
    luminance_rec709_u8_random2: (254, 1, 128) -> 0.25081256,

];
