use crate::{Color4, Linear};
use alexandria_math::number::{FromF32, IntoF32};
use std::marker::Destruct;

impl<T> Color4<T, Linear> {
    /// Composite `self` over `b`
    pub const fn over(self, b: Color4<T, Linear>) -> Color4<T, Linear>
    where
        T: [const] Destruct + [const] IntoF32 + [const] FromF32,
    {
        let (ca, aa) = self.into_f32().rgb_a();
        let (cb, ab) = b.into_f32().rgb_a();

        let ao = aa + ab * (1.0 - aa);
        if ao == 0.0 {
            return Color4::from_f32(Color4::CLEAR);
        }

        Color4::from_f32(((ca * aa + cb * ab * (1.0 - aa)) / ao).with_alpha(ao))
    }

    /// Composite `self` under `b`
    pub const fn under(self, b: Color4<T, Linear>) -> Color4<T, Linear>
    where
        T: [const] Destruct + [const] IntoF32 + [const] FromF32,
    {
        b.over(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Color4, Linear};

    macro_rules! over_under_tests {
        [
            $type: ty = $epsilon: literal:
            $(
                $test_name: ident:
                    ($i1r: literal, $i1g: literal, $i1b: literal, $i1a: literal),
                    ($i2r: literal, $i2g: literal, $i2b: literal, $i2a: literal)
                     -> ($or: literal, $og: literal, $ob: literal, $oa: literal),
            )*
        ] => {$(
            #[test]
            fn $test_name() {
                const EPSILON: $type = $epsilon;

                const INPUT1: Color4<$type, Linear> = Color4::new($i1r, $i1g, $i1b, $i1a);
                const INPUT2: Color4<$type, Linear> = Color4::new($i2r, $i2g, $i2b, $i2a);
                const OUTPUT: Color4<$type, Linear> = Color4::new($or, $og, $ob, $oa);

                let output = INPUT1.over(INPUT2);

                assert!(output.approx_eq(OUTPUT, EPSILON), "over failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    over_under_tests![
        f32 = 1e-6:
        over_under_opaque_over_opaque_keeps_top: (0.2, 0.4, 0.6, 1.0), (0.9, 0.1, 0.3, 1.0) -> (0.2, 0.4, 0.6, 1.0),
        over_under_opaque_over_translucent_keeps_top: (0.2, 0.4, 0.6, 1.0), (0.9, 0.1, 0.3, 0.2) -> (0.2, 0.4, 0.6, 1.0),
        over_under_transparent_over_keeps_under: (0.2, 0.4, 0.6, 0.0), (0.9, 0.1, 0.3, 0.7) -> (0.9, 0.1, 0.3, 0.7),
        over_under_over_transparent_under_keeps_over: (0.2, 0.4, 0.6, 0.5), (0.9, 0.1, 0.3, 0.0) -> (0.2, 0.4, 0.6, 0.5),

        over_under_half_red_over_half_blue: (1.0, 0.0, 0.0, 0.5), (0.0, 0.0, 1.0, 0.5) -> (0.666667, 0.0, 0.333333, 0.75),
        over_under_quarter_green_over_three_quarter_white: (0.0, 1.0, 0.0, 0.25), (1.0, 1.0, 1.0, 0.75) -> (0.692308, 1.0, 0.692308, 0.8125),
        over_under_mostly_opaque_over_translucent_mix: (0.1, 0.2, 0.3, 0.8), (0.7, 0.6, 0.5, 0.4) -> (0.154545, 0.236364, 0.318182, 0.88),
        over_under_low_alpha_over_high_alpha: (0.8, 0.2, 0.1, 0.3), (0.2, 0.9, 0.4, 0.9) -> (0.393548, 0.674194, 0.303226, 0.93),

        over_under_tiny_over_high_alpha: (0.33, 0.66, 0.99, 0.1), (0.99, 0.66, 0.33, 0.9) -> (0.917473, 0.66, 0.402527, 0.91),
        over_under_black_half_over_white_half: (0.0, 0.0, 0.0, 0.5), (1.0, 1.0, 1.0, 0.5) -> (0.333333, 0.333333, 0.333333, 0.75),
        over_under_white_half_over_black_half: (1.0, 1.0, 1.0, 0.5), (0.0, 0.0, 0.0, 0.5) -> (0.666667, 0.666667, 0.666667, 0.75),

        over_under_both_fully_transparent_zero: (0.1, 0.2, 0.3, 0.0), (0.4, 0.5, 0.6, 0.0) -> (0.0, 0.0, 0.0, 0.0),
        over_under_thirty_pct_over_opaque_under: (0.1, 0.2, 0.3, 0.3), (0.4, 0.5, 0.6, 1.0) -> (0.31, 0.41, 0.51, 1.0),
        over_under_high_alpha_over_low_alpha_under: (0.9, 0.9, 0.1, 0.7), (0.2, 0.3, 0.4, 0.2) -> (0.844737, 0.852632, 0.123684, 0.76),
        over_under_one_pct_over_ninety_nine_pct_under: (0.05, 0.95, 0.5, 0.01), (0.9, 0.1, 0.2, 0.99) -> (0.891415, 0.108585, 0.20303, 0.9901),
        over_under_same_color_half_over_half: (0.25, 0.5, 0.75, 0.5), (0.25, 0.5, 0.75, 0.5) -> (0.25, 0.5, 0.75, 0.75),
    ];

    over_under_tests![
        u8 = 0:
        over_under_u8_opaque_over_opaque_keeps_top: (51, 102, 153, 255), (230, 26, 76, 255) -> (51, 102, 153, 255),
        over_under_u8_transparent_over_keeps_under: (51, 102, 153, 0), (230, 25, 76, 178) -> (230, 25, 76, 178),

        over_under_u8_half_red_over_half_blue: (255, 0, 0, 128), (0, 0, 255, 128) -> (170, 0, 85, 192),
        over_under_u8_black_half_over_white_half: (0, 0, 0, 128), (255, 255, 255, 128) -> (85, 85, 85, 192),

        over_under_u8_thirty_pct_over_opaque_under: (26, 51, 76, 76), (102, 128, 153, 255) -> (79, 105, 130, 255),
        over_under_u8_one_pct_over_ninety_nine_pct_under: (13, 242, 128, 3), (230, 26, 51, 252) -> (227, 29, 52, 252),

        over_under_u8_mostly_opaque_over_translucent_mix: (26, 51, 76, 204), (178, 153, 128, 102) -> (40, 60, 81, 224),
        over_under_u8_same_color_half_over_half: (64, 128, 191, 128), (64, 128, 191, 128) -> (64, 128, 191, 192),
    ];
}
