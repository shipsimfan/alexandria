use crate::{Color4, Linear};
use alexandria_math::number::{FromF32, IntoF32};
use std::marker::Destruct;

impl<T: Destruct + FromF32 + IntoF32> Color4<T, Linear> {
    /// Adjust the exposure of this color by `ev`
    pub fn exposure(self, ev: f32) -> Self {
        let (color, a) = self.rgb_a();
        color.exposure(ev).with_alpha(a)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Color4, Linear};

    macro_rules! exposure_tests {
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

                let output = INPUT.exposure($ev);

                assert!(output.approx_eq(OUTPUT, EPSILON), "exposure failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    exposure_tests![
        f32 = 1e-6:
        exposure_identity: (0.1, 0.2, 0.3, 0.4), 0.0 -> (0.1, 0.2, 0.3, 0.4),
        exposure_plus1: (0.1, 0.2, 0.3, 0.4), 1.0 -> (0.2, 0.4, 0.6, 0.4),
        exposure_minus1: (0.1, 0.2, 0.3, 0.4), -1.0 -> (0.05, 0.1, 0.15, 0.4),

        exposure_plus2_with_negative: (1.0, -0.5, 0.0, 1.0), 2.0 -> (4.0, -2.0, 0.0, 1.0),

        exposure_fractional_half: (0.25, 0.5, 0.75, 0.125), 0.5 -> (0.35355338, 0.70710677, 1.0606601, 0.125),
        exposure_fractional_neg_quarter: (2.0, 1.0, 0.5, 0.0), -0.25 -> (1.6817929, 0.8408964, 0.4204482, 0.0),

        exposure_rgb_zeros_alpha_preserved: (0.0, 0.0, 0.0, 0.7), 3.0 -> (0.0, 0.0, 0.0, 0.7),

        exposure_large_ev: (0.001, 1.0, 1000.0, 0.9), 10.0 -> (1.024, 1024.0, 1024000.0, 0.9),

        exposure_tiny_magnitudes: (1e-38, -1e-38, 1e-20, 0.2), 1.0 -> (2e-38, -2e-38, 2e-20, 0.2),
    ];

    exposure_tests![
        u8 = 0:
        exposure_u8_identity: (26, 51, 77, 102), 0.0 -> (26, 51, 77, 102),

        exposure_u8_plus1: (26, 51, 77, 102), 1.0 -> (52, 102, 154, 102),
        exposure_u8_minus1: (26, 51, 77, 102), -1.0 -> (13, 26, 39, 102),

        exposure_u8_plus2_with_saturation: (64, 128, 0, 255), 2.0 -> (255, 255, 0, 255),

        exposure_u8_rgb_zeros_alpha_preserved: (0, 0, 0, 179), 3.0 -> (0, 0, 0, 179),

        exposure_u8_large_ev_saturates: (0, 255, 255, 230), 10.0 -> (0, 255, 255, 230),
    ];
}
