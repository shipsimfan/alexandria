use crate::{
    graphics::color::{Color3, Linear},
    math::number::{FromF32, IntoF32},
};

impl<T: FromF32 + IntoF32> Color3<T, Linear> {
    /// Tone map this color using the Reinhard algorithm
    pub fn tone_map_reinhard(self, exposure: f32) -> Self {
        let color = self.into_f32() * 2f32.powf(exposure);
        let luminance = color.luminance_rec709();
        Color3::from_f32(color.into_f32() / (1.0 + luminance))
    }
}

#[cfg(test)]
mod tests {
    use crate::graphics::color::{Color3, Linear};

    macro_rules! tone_map_reinhard_tests {
        [
            $type: ty = $epsilon: literal:
            $(
                $test_name: ident: ($ir: literal, $ig: literal, $ib: literal), $ev: literal -> ($or: literal, $og: literal, $ob: literal),
            )*
        ] => {$(
            #[test]
            fn $test_name() {
                const EPSILON: $type = $epsilon;

                const INPUT: Color3<$type, Linear> = Color3::new($ir, $ig, $ib);
                const OUTPUT: Color3<$type, Linear> = Color3::new($or, $og, $ob);

                let output = INPUT.tone_map_reinhard($ev);

                assert!(output.approx_eq(OUTPUT, EPSILON), "Reinhard tone map failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    tone_map_reinhard_tests![
        f32 = 1e-6:
        tone_map_reinhard_black_ev0: (0.0, 0.0, 0.0), 0.0 -> (0.000000, 0.000000, 0.000000),

        tone_map_reinhard_midgrey_ev0: (0.18, 0.18, 0.18), 0.0 -> (0.152542, 0.152542, 0.152542),
        tone_map_reinhard_midgrey_ev1: (0.18, 0.18, 0.18), 1.0 -> (0.264706, 0.264706, 0.264706),

        tone_map_reinhard_white_ev0: (1.0, 1.0, 1.0), 0.0 -> (0.500000, 0.500000, 0.500000),

        tone_map_reinhard_red_ev0: (1.0, 0.0, 0.0), 0.0 -> (0.824674, 0.000000, 0.000000),
        tone_map_reinhard_green_ev0: (0.0, 1.0, 0.0), 0.0 -> (0.000000, 0.583022, 0.000000),
        tone_map_reinhard_blue_ev0: (0.0, 0.0, 1.0), 0.0 -> (0.000000, 0.000000, 0.932662),

        tone_map_reinhard_warm_hdr_ev0: (4.0, 2.0, 1.0), 0.0 -> (1.192962, 0.596481, 0.298240),
        tone_map_reinhard_cool_hdr_ev0: (1.0, 2.0, 4.0), 0.0 -> (0.341087, 0.682175, 1.364350),

        tone_map_reinhard_mixed_ev_neg1: (0.5, 1.0, 2.0), -1.0 -> (0.168583, 0.337166, 0.674332),
        tone_map_reinhard_mixed_ev2: (0.5, 1.0, 2.0), 2.0 -> (0.411218, 0.822436, 1.644872),

        tone_map_reinhard_very_bright_ev0: (10.0, 10.0, 10.0), 0.0 -> (0.909091, 0.909091, 0.909091),

        tone_map_reinhard_chromatic_extreme_ev0: (20.0, 1.0, 1.0), 0.0 -> (3.311587, 0.165579, 0.165579),

        tone_map_reinhard_dark_color_ev3: (0.01, 0.02, 0.03), 3.0 -> (0.069640, 0.139280, 0.208919),
    ];

    tone_map_reinhard_tests![
        u8 = 1:
        tone_map_reinhard_black_ev0_u8: (0, 0, 0), 0.0 -> (0, 0, 0),

        tone_map_reinhard_midgrey_ev0_u8: (46, 46, 46), 0.0 -> (39, 39, 39),
        tone_map_reinhard_midgrey_ev1_u8: (46, 46, 46), 1.0 -> (68, 68, 68),

        tone_map_reinhard_white_ev0_u8: (255, 255, 255), 0.0 -> (128, 128, 128),

        tone_map_reinhard_red_ev0_u8: (255, 0, 0), 0.0 -> (210, 0, 0),
        tone_map_reinhard_green_ev0_u8: (0, 255, 0), 0.0 -> (0, 149, 0),
        tone_map_reinhard_blue_ev0_u8: (0, 0, 255), 0.0 -> (0, 0, 238),

        tone_map_reinhard_dark_color_ev3_u8: (3, 5, 8), 3.0 -> (20, 34, 55),
    ];
}
