use crate::{Color4, Linear};
use alexandria_math::number::{FromF32, IntoF32};

impl<T: FromF32 + IntoF32> Color4<T, Linear> {
    /// Tone map this color using the Reinhard algorithm
    pub fn tone_map_reinhard(self, ev: f32) -> Self {
        let (color, a) = self.rgb_a();
        color.tone_map_reinhard(ev).with_alpha(a)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Color4, Linear};

    macro_rules! tone_map_reinhard_tests {
        [
            $type: ty = $epsilon: literal:
            $(
                $test_name: ident: ($ir: literal, $ig: literal, $ib: literal, $ia: literal), $ev: literal -> ($or: literal, $og: literal, $ob: literal, $oa: literal),
            )*
        ] => {$(
            #[test]
            fn $test_name() {
                const EPSILON: $type = $epsilon;

                const INPUT: Color4<$type, Linear> = Color4::new($ir, $ig, $ib, $ia);
                const OUTPUT: Color4<$type, Linear> = Color4::new($or, $og, $ob, $oa);

                let output = INPUT.tone_map_reinhard($ev);

                assert!(output.approx_eq(OUTPUT, EPSILON), "Reinhard tone map failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    tone_map_reinhard_tests![
        f32 = 1e-6:
        tone_map_reinhard_identity_zero: (0.000000, 0.000000, 0.000000, 1.000000), 0.0 -> (0.000000, 0.000000, 0.000000, 1.000000),
        tone_map_reinhard_mid_gray_ev0: (0.180000, 0.180000, 0.180000, 1.000000), 0.0 -> (0.152542, 0.152542, 0.152542, 1.000000),
        tone_map_reinhard_mid_gray_ev1: (0.180000, 0.180000, 0.180000, 1.000000), 1.0 -> (0.264706, 0.264706, 0.264706, 1.000000),
        tone_map_reinhard_mid_gray_ev_neg2: (0.180000, 0.180000, 0.180000, 1.000000), -2.0 -> (0.043062, 0.043062, 0.043062, 1.000000),

        tone_map_reinhard_primary_red_ev0: (1.000000, 0.000000, 0.000000, 1.000000), 0.0 -> (0.824674, 0.000000, 0.000000, 1.000000),
        tone_map_reinhard_primary_green_ev0: (0.000000, 1.000000, 0.000000, 1.000000), 0.0 -> (0.000000, 0.583022, 0.000000, 1.000000),
        tone_map_reinhard_primary_blue_ev0: (0.000000, 0.000000, 1.000000, 1.000000), 0.0 -> (0.000000, 0.000000, 0.932662, 1.000000),

        tone_map_reinhard_white_ev0: (1.000000, 1.000000, 1.000000, 1.000000), 0.0 -> (0.500000, 0.500000, 0.500000, 1.000000),
        tone_map_reinhard_white_ev2: (1.000000, 1.000000, 1.000000, 1.000000), 2.0 -> (0.800000, 0.800000, 0.800000, 1.000000),

        tone_map_reinhard_hot_color_ev0: (4.000000, 2.000000, 1.000000, 0.500000), 0.0 -> (1.192962, 0.596481, 0.298240, 0.500000),
        tone_map_reinhard_hot_color_ev_neg1: (4.000000, 2.000000, 1.000000, 0.500000), -1.0 -> (0.918907, 0.459453, 0.229727, 0.500000),

        tone_map_reinhard_negative_rgb_ev0: (-0.500000, 0.200000, 1.000000, 1.000000), 0.0 -> (-0.450881, 0.180352, 0.901762, 1.000000),
        tone_map_reinhard_alpha_passthrough: (0.200000, 0.400000, 0.600000, 0.250000), 0.0 -> (0.145781, 0.291562, 0.437343, 0.250000),

        tone_map_reinhard_hdr10_white_ev0: (10.000000, 10.000000, 10.000000, 1.000000), 0.0 -> (0.909091, 0.909091, 0.909091, 1.000000),
        tone_map_reinhard_hdr10_white_ev_neg4: (10.000000, 10.000000, 10.000000, 1.000000), -4.0 -> (0.384615, 0.384615, 0.384615, 1.000000),
    ];

    tone_map_reinhard_tests![
        u8 = 0:
        tone_map_reinhard_u8_black_ev0: (0, 0, 0, 255), 0.0 -> (0, 0, 0, 255),
        tone_map_reinhard_u8_midgray46_ev0: (46, 46, 46, 255), 0.0 -> (39, 39, 39, 255),

        tone_map_reinhard_u8_primary_red_ev0: (255, 0, 0, 255), 0.0 -> (210, 0, 0, 255),
        tone_map_reinhard_u8_primary_green_ev0: (0, 255, 0, 255), 0.0 -> (0, 149, 0, 255),
        tone_map_reinhard_u8_primary_blue_ev0: (0, 0, 255, 255), 0.0 -> (0, 0, 238, 255),

        tone_map_reinhard_u8_white_ev0: (255, 255, 255, 255), 0.0 -> (128, 128, 128, 255),
        tone_map_reinhard_u8_white_ev2: (255, 255, 255, 255), 2.0 -> (204, 204, 204, 255),

        tone_map_reinhard_u8_hot_color_ev0: (255, 128, 64, 128), 0.0 -> (160, 81, 40, 128),
        tone_map_reinhard_u8_alpha_passthrough_ev0: (51, 102, 153, 64), 0.0 -> (37, 74, 112, 64),

        tone_map_reinhard_u8_white_ev_neg4: (255, 255, 255, 255), -4.0 -> (15, 15, 15, 255),
    ];
}
