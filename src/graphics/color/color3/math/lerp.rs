use crate::{
    graphics::color::{Color3, Linear},
    math::number::{FromF32, IntoF32},
};
use std::marker::Destruct;

impl<T> Color3<T, Linear> {
    /// Linear interpolate between this color and another color, using a parameter
    pub const fn lerp(self, b: Self, t: f32) -> Self
    where
        T: [const] FromF32 + [const] IntoF32 + [const] Destruct,
    {
        Color3::new(
            T::from_f32((1.0 - t) * self.r.into_f32() + t * b.r.into_f32()),
            T::from_f32((1.0 - t) * self.g.into_f32() + t * b.g.into_f32()),
            T::from_f32((1.0 - t) * self.b.into_f32() + t * b.b.into_f32()),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::graphics::color::{Color3, Linear};

    macro_rules! lerp_tests {
        [
            $type: ty = $epsilon: literal:
            $(
                $test_name: ident:
                ($i1r: literal, $i1g: literal, $i1b: literal),
                    ($i2r: literal, $i2g: literal, $i2b: literal),
                    $t: literal
                    -> ($or: literal, $og: literal, $ob: literal),
            )*
        ] => {$(
            #[test]
            fn $test_name() {
                const EPSILON: $type = $epsilon;

                const INPUT1: Color3<$type, Linear> = Color3::new($i1r, $i1g, $i1b);
                const INPUT2: Color3<$type, Linear> = Color3::new($i2r, $i2g, $i2b);
                const OUTPUT: Color3<$type, Linear> = Color3::new($or, $og, $ob);

                let output = INPUT1.lerp(INPUT2, $t);

                assert!(output.approx_eq(OUTPUT, EPSILON), "lerp failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    lerp_tests![
        f32 = 1e-6:
        lerp_t0_returns_start: (0.2, 0.4, 0.6), (0.9, 0.1, 0.3), 0.0 -> (0.2, 0.4, 0.6),
        lerp_t1_returns_end: (0.2, 0.4, 0.6), (0.9, 0.1, 0.3), 1.0 -> (0.9, 0.1, 0.3),
        lerp_halfway_midpoint: (0.0, 0.0, 0.0), (1.0, 1.0, 1.0), 0.5 -> (0.5, 0.5, 0.5),
        lerp_quarter_to_white: (0.2, 0.4, 0.6), (1.0, 1.0, 1.0), 0.25 -> (0.4, 0.55, 0.7),
        lerp_three_quarters_to_target: (0.1, 0.8, 0.3), (0.9, 0.2, 0.7), 0.75 -> (0.7, 0.35, 0.6),
        lerp_channel_crossing: (1.0, 0.0, 0.0), (0.0, 1.0, 0.0), 0.25 -> (0.75, 0.25, 0.0),
        lerp_nontrivial_fraction: (0.33, 0.66, 0.99), (0.11, 0.22, 0.44), 0.6 -> (0.198, 0.396, 0.66),
        lerp_small_alpha: (0.9, 0.2, 0.1), (0.0, 0.0, 1.0), 0.1 -> (0.81, 0.18, 0.19),
        lerp_large_alpha: (0.9, 0.2, 0.1), (0.0, 0.0, 1.0), 0.9 -> (0.09, 0.02, 0.91),
        lerp_gray_to_color: (0.5, 0.5, 0.5), (0.2, 0.7, 0.9), 0.4 -> (0.38, 0.58, 0.66),
        lerp_rgb_extremes: (1.0, 1.0, 1.0), (0.0, 0.5, 1.0), 0.3 -> (0.7, 0.85, 1.0),
        lerp_blue_to_yellow: (0.0, 0.0, 1.0), (1.0, 1.0, 0.0), 0.5 -> (0.5, 0.5, 0.5),
    ];

    lerp_tests![
        u8 = 1:
        lerp_t0_returns_start_u8: (51, 102, 153), (230, 26, 76), 0.0 -> (51, 102, 153),
        lerp_t1_returns_end_u8: (51, 102, 153), (230, 26, 76), 1.0 -> (230, 26, 76),

        lerp_channel_crossing_u8: (255, 0, 0), (0, 255, 0), 0.25 -> (191, 64, 0),

        lerp_small_alpha_u8: (230, 51, 26), (0, 0, 255), 0.1 -> (207, 46, 48),
        lerp_large_alpha_u8: (230, 51, 26), (0, 0, 255), 0.9 -> (23, 5, 232),

        lerp_gray_to_color_u8: (128, 128, 128), (51, 178, 230), 0.4 -> (97, 148, 168),
    ];
}
