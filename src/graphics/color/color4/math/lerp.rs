use crate::{
    graphics::color::{Color4, Linear},
    math::number::{FromF32, IntoF32},
};
use std::marker::Destruct;

impl<T> Color4<T, Linear> {
    /// Linear interpolate between this color and another color, using a parameter
    pub const fn lerp(self, b: Self, t: f32) -> Self
    where
        T: [const] FromF32 + [const] IntoF32 + [const] Destruct,
    {
        Color4::new(
            T::from_f32((1.0 - t) * self.r.into_f32() + t * b.r.into_f32()),
            T::from_f32((1.0 - t) * self.g.into_f32() + t * b.g.into_f32()),
            T::from_f32((1.0 - t) * self.b.into_f32() + t * b.b.into_f32()),
            T::from_f32((1.0 - t) * self.a.into_f32() + t * b.a.into_f32()),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::graphics::color::{Color4, Linear};

    macro_rules! lerp_tests {
        [
            $type: ty = $epsilon: literal:
            $(
                $test_name: ident:
                ($i1r: literal, $i1g: literal, $i1b: literal, $i1a: literal),
                    ($i2r: literal, $i2g: literal, $i2b: literal, $i2a: literal),
                    $t: literal
                    -> ($or: literal, $og: literal, $ob: literal, $oa: literal),
            )*
        ] => {$(
            #[test]
            fn $test_name() {
                const EPSILON: $type = $epsilon;

                const INPUT1: Color4<$type, Linear> = Color4::new($i1r, $i1g, $i1b, $i1a);
                const INPUT2: Color4<$type, Linear> = Color4::new($i2r, $i2g, $i2b, $i2a);
                const OUTPUT: Color4<$type, Linear> = Color4::new($or, $og, $ob, $oa);

                let output = INPUT1.lerp(INPUT2, $t);

                assert!(output.approx_eq(OUTPUT, EPSILON), "lerp failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    lerp_tests![
        f32 = 1e-6:
        lerp_zero_t: (0.2, 0.4, 0.6, 0.8), (0.9, 0.1, 0.3, 0.5), 0.0 -> (0.2, 0.4, 0.6, 0.8),
        lerp_one_t: (0.2, 0.4, 0.6, 0.8), (0.9, 0.1, 0.3, 0.5), 1.0 -> (0.9, 0.1, 0.3, 0.5),

        lerp_half_t: (0.0, 0.25, 0.5, 0.75), (1.0, 0.75, 0.25, 0.0), 0.5 -> (0.5, 0.5, 0.375, 0.375),
        lerp_quarter_t: (0.0, 0.0, 0.0, 0.0), (1.0, 1.0, 1.0, 1.0), 0.25 -> (0.25, 0.25, 0.25, 0.25),
        lerp_three_quarters_t: (0.0, 0.0, 0.0, 0.0), (1.0, 1.0, 1.0, 1.0), 0.75 -> (0.75, 0.75, 0.75, 0.75),

        lerp_midpoint_mixed: (0.1, 0.9, 0.3, 0.7), (0.9, 0.1, 0.7, 0.3), 0.5 -> (0.5, 0.5, 0.5, 0.5),
        lerp_weighted_20pct: (0.1, 0.9, 0.3, 0.7), (0.9, 0.1, 0.7, 0.3), 0.2 -> (0.26, 0.74, 0.38, 0.62),
        lerp_weighted_80pct: (0.1, 0.9, 0.3, 0.7), (0.9, 0.1, 0.7, 0.3), 0.8 -> (0.74, 0.26, 0.62, 0.38),

        lerp_crossing_zero: (-1.0, 2.0, -3.0, 4.0), (1.0, -2.0, 3.0, -4.0), 0.5 -> (0.0, 0.0, 0.0, 0.0),
        lerp_negative_t_extrapolate: (0.0, 1.0, 2.0, 3.0), (10.0, 11.0, 12.0, 13.0), -0.5 -> (-5.0, -4.0, -3.0, -2.0),
        lerp_over_one_t_extrapolate: (0.0, 1.0, 2.0, 3.0), (10.0, 11.0, 12.0, 13.0), 1.5 -> (15.0, 16.0, 17.0, 18.0),

        lerp_large_values: (1000.0, -1000.0, 0.001, -0.001), (-1000.0, 1000.0, 0.003, -0.003), 0.25 -> (500.0, -500.0, 0.0015, -0.0015),
        lerp_tiny_values: (0.000001, 0.000002, 0.000003, 0.000004), (0.000005, 0.000006, 0.000007, 0.000008), 0.5 -> (0.000003, 0.000004, 0.000005, 0.000006),

        lerp_alpha_only: (0.3, 0.3, 0.3, 0.0), (0.3, 0.3, 0.3, 1.0), 0.6 -> (0.3, 0.3, 0.3, 0.6),
        lerp_rgb_only: (0.0, 0.5, 1.0, 0.25), (1.0, 0.0, 0.5, 0.25), 0.4 -> (0.4, 0.3, 0.8, 0.25),

        lerp_identity_same_inputs: (0.42, 0.24, 0.66, 0.18), (0.42, 0.24, 0.66, 0.18), 0.73 -> (0.42, 0.24, 0.66, 0.18),
        lerp_endpoints_symmetry_check: (0.15, 0.25, 0.35, 0.45), (0.85, 0.75, 0.65, 0.55), 0.3 -> (0.36, 0.4, 0.44, 0.48),
    ];

    lerp_tests![
        u8 = 1:
        lerp_u8_zero_t: (10, 20, 30, 40), (200, 150, 100, 50), 0.0 -> (10, 20, 30, 40),
        lerp_u8_one_t: (10, 20, 30, 40), (200, 150, 100, 50), 1.0 -> (200, 150, 100, 50),

        lerp_u8_half_t_even_deltas: (0, 64, 128, 192), (100, 164, 28, 92), 0.5 -> (50, 114, 78, 142),
        lerp_u8_half_t_black_to_white: (0, 0, 0, 0), (254, 254, 254, 254), 0.5 -> (127, 127, 127, 127),

        lerp_u8_quarter_t_multiples_of_4: (0, 40, 80, 120), (200, 240, 120, 160), 0.25 -> (50, 90, 90, 130),
        lerp_u8_three_quarters_t_multiples_of_4: (0, 40, 80, 120), (200, 240, 120, 160), 0.75 -> (150, 190, 110, 150),

        lerp_u8_midpoint_mixed: (25, 225, 75, 175), (225, 25, 175, 75), 0.5 -> (125, 125, 125, 125),
        lerp_u8_weighted_20pct_multiples_of_5: (10, 90, 30, 70), (210, 10, 130, 30), 0.2 -> (50, 74, 50, 62),
        lerp_u8_weighted_80pct_multiples_of_5: (10, 90, 30, 70), (210, 10, 130, 30), 0.8 -> (170, 26, 110, 38),

        lerp_u8_alpha_only: (77, 77, 77, 0), (77, 77, 77, 200), 0.5 -> (77, 77, 77, 100),
        lerp_u8_rgb_only: (0, 128, 255, 64), (200, 0, 55, 64), 0.25 -> (50, 96, 205, 64),
    ];
}
