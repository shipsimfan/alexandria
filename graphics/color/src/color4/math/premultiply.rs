use crate::{Color4, Linear};
use alexandria_math::number::{FromF32, IntoF32};
use std::marker::Destruct;

impl<T> Color4<T, Linear> {
    /// Premultiply the color channels by the alpha channel
    pub const fn premultiply(self) -> Color4<T, Linear>
    where
        T: [const] Destruct + [const] IntoF32 + [const] FromF32,
    {
        let (color, a) = self.into_f32().rgb_a();
        Color4::from_f32((color * a).with_alpha(a))
    }

    /// Unpremulitply the color channels by the alpha channel
    pub const fn unpremultiply(self) -> Color4<T, Linear>
    where
        T: [const] Destruct + [const] IntoF32 + [const] FromF32,
    {
        let (color, a) = self.into_f32().rgb_a();
        if a == 0.0 {
            return Color4::from_f32(Color4::CLEAR);
        }
        Color4::from_f32((color / a).with_alpha(a))
    }
}

#[cfg(test)]
mod tests {
    use crate::{Color4, Linear};
    use alexandria_math::number::Zero;

    macro_rules! premultiply_tests {
        [
            $type: ty = $epsilon: literal:
            $(
                $test_name: ident: ($ir: literal, $ig: literal, $ib: literal, $ia: literal) -> ($or: literal, $og: literal, $ob: literal, $oa: literal),
            )*
        ] => {$(
            #[test]
            fn $test_name() {
                const EPSILON: $type = $epsilon;

                const STRAIGHT: Color4<$type, Linear> = Color4::new($ir, $ig, $ib, $ia);
                const PREMULTIPLIED: Color4<$type, Linear> = Color4::new($or, $og, $ob, $oa);

                let premultiplied = STRAIGHT.premultiply();

                assert!(premultiplied.approx_eq(PREMULTIPLIED, EPSILON), "premultiply failed: {} vs. {}", premultiplied, PREMULTIPLIED);

                if STRAIGHT.a != <$type>::ZERO {
                    let straight = PREMULTIPLIED.unpremultiply();
                    assert!(straight.approx_eq(STRAIGHT, EPSILON), "unpremultiply failed: {} vs. {}", straight, STRAIGHT);
                }
            }
        )*};
    }

    premultiply_tests![
        f32 = 1e-6:
        premultiply_identity_alpha1: (0.2, 0.4, 0.6, 1.0) -> (0.2, 0.4, 0.6, 1.0),
        premultiply_zero_alpha: (0.2, 0.4, 0.6, 0.0) -> (0.0, 0.0, 0.0, 0.0),
        premultiply_zero_rgb_alpha1: (0.0, 0.0, 0.0, 1.0) -> (0.0, 0.0, 0.0, 1.0),
        premultiply_zero_all: (0.0, 0.0, 0.0, 0.0) -> (0.0, 0.0, 0.0, 0.0),

        premultiply_half_alpha: (0.2, 0.4, 0.6, 0.5) -> (0.1, 0.2, 0.3, 0.5),
        premultiply_quarter_alpha: (0.8, 0.4, 0.2, 0.25) -> (0.2, 0.1, 0.05, 0.25),
        premultiply_three_quarters_alpha: (0.8, 0.4, 0.2, 0.75) -> (0.6, 0.3, 0.15, 0.75),

        premultiply_full_red_half_alpha: (1.0, 0.0, 0.0, 0.5) -> (0.5, 0.0, 0.0, 0.5),
        premultiply_full_green_quarter_alpha: (0.0, 1.0, 0.0, 0.25) -> (0.0, 0.25, 0.0, 0.25),
        premultiply_full_blue_three_quarters_alpha: (0.0, 0.0, 1.0, 0.75) -> (0.0, 0.0, 0.75, 0.75),

        premultiply_mixed_exact_binary_1: (0.5, 0.25, 0.75, 0.5) -> (0.25, 0.125, 0.375, 0.5),
        premultiply_mixed_exact_binary_2: (0.75, 0.5, 0.25, 0.25) -> (0.1875, 0.125, 0.0625, 0.25),
        premultiply_mixed_exact_binary_3: (0.25, 0.75, 0.5, 0.75) -> (0.1875, 0.5625, 0.375, 0.75),

        premultiply_alpha_gt_one: (0.2, 0.4, 0.6, 2.0) -> (0.4, 0.8, 1.2, 2.0),
        premultiply_rgb_gt_one: (2.0, 1.5, 1.25, 0.5) -> (1.0, 0.75, 0.625, 0.5),

        premultiply_negative_alpha: (0.2, 0.4, 0.6, -0.5) -> (-0.1, -0.2, -0.3, -0.5),
        premultiply_negative_rgb: (-0.2, 0.4, -0.6, 0.5) -> (-0.1, 0.2, -0.3, 0.5),
        premultiply_negative_both: (-1.0, 2.0, -3.0, -0.5) -> (0.5, -1.0, 1.5, -0.5),

        premultiply_preserves_alpha_zero: (1.0, 1.0, 1.0, 0.0) -> (0.0, 0.0, 0.0, 0.0),
        premultiply_preserves_alpha_half: (1.0, 1.0, 1.0, 0.5) -> (0.5, 0.5, 0.5, 0.5),
        premultiply_preserves_alpha_one: (1.0, 1.0, 1.0, 1.0) -> (1.0, 1.0, 1.0, 1.0),

        premultiply_large_values: (1000.0, -1000.0, 0.5, 0.5) -> (500.0, -500.0, 0.25, 0.5),
        premultiply_small_values: (1.0e-6, 2.0e-6, -4.0e-6, 0.5) -> (5.0e-7, 1.0e-6, -2.0e-6, 0.5),
    ];

    premultiply_tests![
        u8 = 0:
        premultiply_u8_identity_alpha255: (51, 102, 153, 255) -> (51, 102, 153, 255),
        premultiply_u8_zero_alpha0: (51, 102, 153, 0) -> (0, 0, 0, 0),
        premultiply_u8_zero_rgb_alpha255: (0, 0, 0, 255) -> (0, 0, 0, 255),
        premultiply_u8_zero_all: (0, 0, 0, 0) -> (0, 0, 0, 0),

        premultiply_u8_half_alpha128_powers_of_two: (64, 128, 191, 128) -> (32, 64, 96, 128),
        premultiply_u8_quarter_alpha64_powers_of_two: (64, 128, 191, 64) -> (16, 32, 48, 64),
        premultiply_u8_three_quarters_alpha192_powers_of_two: (64, 128, 193, 192) -> (48, 96, 145, 192),

        premultiply_u8_full_red_half_alpha128: (255, 0, 0, 128) -> (128, 0, 0, 128),
        premultiply_u8_full_green_quarter_alpha64: (0, 255, 0, 64) -> (0, 64, 0, 64),
        premultiply_u8_full_blue_three_quarters_alpha192: (0, 0, 255, 192) -> (0, 0, 192, 192),

        premultiply_u8_white_half_alpha128: (255, 255, 255, 128) -> (128, 128, 128, 128),
        premultiply_u8_white_alpha0: (255, 255, 255, 0) -> (0, 0, 0, 0),
        premultiply_u8_white_alpha255: (255, 255, 255, 255) -> (255, 255, 255, 255),

        premultiply_u8_midgray_half_alpha128: (128, 128, 128, 128) -> (64, 64, 64, 128),
        premultiply_u8_varied_half_alpha128: (199, 100, 50, 128) -> (100, 50, 25, 128),
        premultiply_u8_varied_quarter_alpha64: (199, 100, 52, 64) -> (50, 25, 13, 64),
    ];
}
