use crate::{
    graphics::color::{Color3, Linear},
    math::number::{FromF32, IntoF32},
};
use std::marker::Destruct;

impl<T: Destruct + FromF32 + IntoF32> Color3<T, Linear> {
    /// Adjust the exposure of this color by `ev`
    pub fn exposure(self, ev: f32) -> Self {
        Color3::from_f32(self.into_f32() * 2.0f32.powf(ev))
    }
}

#[cfg(test)]
mod tests {
    use crate::graphics::color::{Color3, Linear};

    macro_rules! exposure_tests {
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

                let output = INPUT.exposure($ev);

                assert!(output.approx_eq(OUTPUT, EPSILON), "exposure failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    exposure_tests![
        f32 = 1e-6:
        exposure_identity_0ev: (0.25, 0.50, 0.75), 0.0 -> (0.25, 0.50, 0.75),
        exposure_black_any_ev_pos: (0.0, 0.0, 0.0), 5.0 -> (0.0, 0.0, 0.0),
        exposure_black_any_ev_neg: (0.0, 0.0, 0.0), -5.0 -> (0.0, 0.0, 0.0),

        exposure_plus1ev_doubles: (0.10, 0.20, 0.30), 1.0 -> (0.20, 0.40, 0.60),
        exposure_plus2ev_quads: (0.10, 0.20, 0.30), 2.0 -> (0.40, 0.80, 1.20),
        exposure_plus3ev_eightx: (0.125, 0.25, 0.5), 3.0 -> (1.0, 2.0, 4.0),

        exposure_minus1ev_halves: (0.10, 0.20, 0.30), -1.0 -> (0.05, 0.10, 0.15),
        exposure_minus2ev_quarter: (0.10, 0.20, 0.30), -2.0 -> (0.025, 0.05, 0.075),
        exposure_minus3ev_eighth: (1.0, 2.0, 4.0), -3.0 -> (0.125, 0.25, 0.5),

        exposure_fractional_plus0_5ev_sqrt2: (1.0, 2.0, 4.0), 0.5 -> (1.4142135623730951, 2.8284271247461903, 5.656854249492381),
        exposure_fractional_minus0_5ev_inv_sqrt2: (1.0, 2.0, 4.0), -0.5 -> (0.7071067811865476, 1.4142135623730951, 2.8284271247461903),

        exposure_fractional_plus0_25ev_2pow0_25: (0.5, 1.0, 2.0), 0.25 -> (0.5946035575013605, 1.189207115002721, 2.378414230005442),
        exposure_fractional_minus0_25ev_2powm0_25: (0.5, 1.0, 2.0), -0.25 -> (0.42044820762685725, 0.8408964152537145, 1.681792830507429),

        exposure_nonuniform_input_uniform_gain: (0.0, 0.25, 1.0), 2.0 -> (0.0, 1.0, 4.0),
        exposure_negative_values_preserved: (-0.25, 0.5, -1.0), 1.0 -> (-0.5, 1.0, -2.0),
        exposure_mixed_sign_fractional: (-1.0, 0.0, 1.0), -1.5 -> (-0.3535533905932738, 0.0, 0.3535533905932738),

        exposure_large_ev_scaling: (1.0, 0.5, 0.25), 10.0 -> (1024.0, 512.0, 256.0),
        exposure_small_ev_scaling: (1024.0, 512.0, 256.0), -10.0 -> (1.0, 0.5, 0.25),
    ];

    exposure_tests![
        u8 = 1:
        exposure_u8_identity_0ev: (64, 128, 191), 0.0 -> (64, 128, 191),
        exposure_u8_black_any_ev_pos: (0, 0, 0), 5.0 -> (0, 0, 0),
        exposure_u8_black_any_ev_neg: (0, 0, 0), -5.0 -> (0, 0, 0),

        exposure_u8_plus1ev_doubles: (26, 51, 77), 1.0 -> (51, 102, 154),
        exposure_u8_minus1ev_halves: (26, 51, 77), -1.0 -> (13, 26, 38),

        exposure_u8_nonuniform_input_uniform_gain_ev2: (0, 64, 255), 2.0 -> (0, 255, 255),

        exposure_u8_fractional_plus0_5ev: (255, 255, 255), 0.5 -> (255, 255, 255),
        exposure_u8_fractional_minus0_5ev: (255, 255, 255), -0.5 -> (180, 180, 180),

        exposure_u8_large_ev_pos_clamps: (255, 128, 64), 2.0 -> (255, 255, 255),
        exposure_u8_large_ev_neg: (255, 128, 64), -2.0 -> (64, 32, 16),
    ];
}
