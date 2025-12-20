use crate::{Color4, Linear};
use alexandria_math::{Vector3, number::IntoF32};
use std::marker::Destruct;

impl<T> Color4<T, Linear> {
    /// Calculate the Rec.709 luminance for this color
    pub const fn luminance_rec709(self) -> f32
    where
        T: [const] Destruct + [const] IntoF32,
    {
        self.luminance(Vector3::new(0.2126, 0.7152, 0.0722))
    }
}

#[cfg(test)]
mod tests {
    use crate::{Color4, Linear};
    use alexandria_math::number::ApproxEq;

    macro_rules! luminance_tests {
        [
            $type: ty:
            $(
                $test_name: ident: ($ir: literal, $ig: literal, $ib: literal, $ia: literal) -> $luminance: literal,
            )*
        ] => {$(
            #[test]
            fn $test_name() {
                const INPUT: Color4<$type, Linear> = Color4::new($ir, $ig, $ib, $ia);
                const OUTPUT: f32 = $luminance;

                let output = INPUT.luminance_rec709();

                assert!(output.approx_eq(OUTPUT, 1e-6), "luminance rec709 failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    luminance_tests![
        f32:
        luminance_rec709_red_1: (1.0, 0.0, 0.0, 1.0) -> 0.2126,
        luminance_rec709_green_1: (0.0, 1.0, 0.0, 1.0) -> 0.7152,
        luminance_rec709_blue_1: (0.0, 0.0, 1.0, 1.0) -> 0.0722,

        luminance_rec709_red_half: (0.5, 0.0, 0.0, 0.0) -> 0.1063,
        luminance_rec709_green_half: (0.0, 0.5, 0.0, 0.0) -> 0.3576,
        luminance_rec709_blue_half: (0.0, 0.0, 0.5, 0.0) -> 0.0361,

        luminance_rec709_white_alpha_ignored: (1.0, 1.0, 1.0, 0.3) -> 1.0,
        luminance_rec709_gray_quarter: (0.25, 0.25, 0.25, 1.0) -> 0.25,

        luminance_rec709_mix_0p2_0p4_0p6: (0.2, 0.4, 0.6, 1.0) -> 0.37192,
        luminance_rec709_mix_redheavy: (0.9, 0.1, 0.0, 0.0) -> 0.26286,
        luminance_rec709_mix_blueheavy: (0.0, 0.1, 0.9, 1.0) -> 0.1365,

        luminance_rec709_alpha_zero: (0.3, 0.6, 0.9, 0.0) -> 0.55788,
        luminance_rec709_hdr_red_2: (2.0, 0.0, 0.0, 1.0) -> 0.4252,
    ];

    luminance_tests![
        u8:
        luminance_rec709_u8_red_255: (255, 0, 0, 255) -> 0.2126,
        luminance_rec709_u8_green_255: (0, 255, 0, 255) -> 0.7152,
        luminance_rec709_u8_blue_255: (0, 0, 255, 255) -> 0.0722,

        luminance_rec709_u8_white_alpha_ignored: (255, 255, 255, 77) -> 1.0,
        luminance_rec709_u8_gray_64: (64, 64, 64, 255) -> 0.2509804,

        luminance_rec709_u8_mix_51_102_153: (51, 102, 153, 255) -> 0.37192,
        luminance_rec709_u8_mix_redheavy_230_26_0: (230, 26, 0, 0) -> 0.26467922,
        luminance_rec709_u8_mix_blueheavy_0_26_230: (0, 26, 230, 255) -> 0.13804393,

        luminance_rec709_u8_alpha_zero_77_153_230: (77, 153, 230, 0) -> 0.5584384,
    ];
}
