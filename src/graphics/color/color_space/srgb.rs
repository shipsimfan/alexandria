use crate::{
    graphics::color::{Color3, ColorSpace, Linear},
    math::number::{FromF32, IntoF32},
};

/// sRGB-encoded RGB (standard display transfer function)
///
/// # Meaning
/// [`Srgb`] indicates that RGB channels are **non-linear, display-referred** values following the
/// sRGB transfer function.
///
/// You typically use [`Srgb`] for:
/// - UI/theme colors
/// - Authoring-friendly “what you see” color pickers
/// - Interop with sRGB textures or 8-bit RGBA assets
///
/// # Important
/// Many operations (blending, lerp, lighting) should **not** be performed in sRGB space. Convert
/// to [`Linear`] first, operate, then convert back if needed.
///
/// # Alpha
/// Alpha is commonly stored alongside sRGB RGB in asset formats, but alpha itself is not
/// “sRGB-encoded”; it’s still a linear coverage value.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Srgb;

/// Convert an individual color `channel` from sRGB to linear
fn channel_into_linear(channel: f32) -> f32 {
    if channel < 0.04045 {
        channel * 0.0773993808
    } else {
        (channel * 0.9478672986 + 0.0521327014).powf(2.4)
    }
}

/// Convert an individual color `channel` from linear to sRGB
fn channel_from_linear(channel: f32) -> f32 {
    if channel < 0.0031308 {
        channel * 12.92
    } else {
        1.055 * channel.powf(1.0 / 2.4) - 0.055
    }
}

impl<T: Sized + FromF32 + IntoF32> ColorSpace<T> for Srgb {
    fn from_linear(color: Color3<T, Linear>) -> Color3<T, Self> {
        unsafe {
            color.map_channels_and_space(|channel| {
                T::from_normalized_f32(channel_from_linear(channel.into_normalized_f32()))
            })
        }
    }

    fn into_linear(color: Color3<T, Self>) -> Color3<T, Linear> {
        unsafe {
            color.map_channels_and_space(|channel| {
                T::from_normalized_f32(channel_into_linear(channel.into_normalized_f32()))
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::graphics::color::{Color3, ColorSpace, Linear, Srgb};

    macro_rules! conversion_tests {
        [
            $type: ty = $epsilon: literal:
            $(
                $test_name: ident: ($lr: literal, $lg: literal, $lb: literal) -> ($sr: literal, $sg: literal, $sb: literal),
            )*
        ] => {$(
            #[test]
            fn $test_name() {
                const EPSILON: $type = $epsilon;

                const LINEAR: Color3<$type, Linear> = Color3::new($lr, $lg, $lb);
                const SRGB: Color3<$type, Srgb> = Color3::new($sr, $sg, $sb);

                let srgb = Srgb::from_linear(LINEAR);
                let linear = Srgb::into_linear(SRGB);

                assert!(srgb.approx_eq(SRGB, EPSILON), "sRGB failed: {} vs. {}", srgb, SRGB);
                assert!(linear.approx_eq(LINEAR, EPSILON), "linear failed: {} vs. {}", linear, LINEAR);
            }
        )*};
    }

    conversion_tests![
        f32 = 1e-6:
        srgb_conversion_linear_black: (0.0, 0.0, 0.0) -> (0.0, 0.0, 0.0),
        srgb_conversion_linear_white: (1.0, 1.0, 1.0) -> (1.0, 1.0, 1.0),
        srgb_conversion_linear_middle_gray_18: (0.18, 0.18, 0.18) -> (0.46135613, 0.46135613, 0.46135613),
        srgb_conversion_linear_half: (0.5, 0.5, 0.5) -> (0.73535698, 0.73535698, 0.73535698),
        srgb_conversion_linear_quarter: (0.25, 0.25, 0.25) -> (0.53709873, 0.53709873, 0.53709873),
        srgb_conversion_linear_three_quarters: (0.75, 0.75, 0.75) -> (0.88082502, 0.88082502, 0.88082502),

        srgb_conversion_linear_toe_threshold: (0.0031308, 0.0031308, 0.0031308) -> (0.04044994, 0.04044994, 0.04044994),
        srgb_conversion_linear_toe_just_below: (0.0031307, 0.0031307, 0.0031307) -> (0.04044864, 0.04044864, 0.04044864),
        srgb_conversion_linear_toe_just_above: (0.0031309, 0.0031309, 0.0031309) -> (0.04045118, 0.04045118, 0.04045118),
        srgb_conversion_linear_toe_small: (0.001, 0.001, 0.001) -> (0.01292, 0.01292, 0.01292),
        srgb_conversion_linear_low_01: (0.01, 0.01, 0.01) -> (0.09985282, 0.09985282, 0.09985282),

        srgb_conversion_linear_red: (1.0, 0.0, 0.0) -> (1.0, 0.0, 0.0),
        srgb_conversion_linear_green: (0.0, 1.0, 0.0) -> (0.0, 1.0, 0.0),
        srgb_conversion_linear_blue: (0.0, 0.0, 1.0) -> (0.0, 0.0, 1.0),
        srgb_conversion_linear_cyan: (0.0, 1.0, 1.0) -> (0.0, 1.0, 1.0),
        srgb_conversion_linear_magenta: (1.0, 0.0, 1.0) -> (1.0, 0.0, 1.0),
        srgb_conversion_linear_yellow: (1.0, 1.0, 0.0) -> (1.0, 1.0, 0.0),

        srgb_conversion_linear_mixed_18_50_01: (0.18, 0.5, 0.01) -> (0.46135613, 0.73535698, 0.09985282),
        srgb_conversion_linear_mixed_04_10_90: (0.04, 0.1, 0.9) -> (0.22091636, 0.34919021, 0.95468717),
        srgb_conversion_linear_mixed_02_75_25: (0.02, 0.75, 0.25) -> (0.15170372, 0.88082502, 0.53709873),
        srgb_conversion_linear_mixed_toe_and_linear: (0.0031308, 0.004, 0.002) -> (0.04044994, 0.05070871, 0.02584),
        srgb_conversion_linear_mixed_high_mid_low: (0.9, 0.1, 0.001) -> (0.95468717, 0.34919021, 0.01292),

        srgb_conversion_srgb_triplet_black: (0.0, 0.0, 0.0) -> (0.0, 0.0, 0.0),
        srgb_conversion_srgb_triplet_white: (1.0, 1.0, 1.0) -> (1.0, 1.0, 1.0),
        srgb_conversion_srgb_triplet_toe_threshold: (0.0031308, 0.0031308, 0.0031308) -> (0.04045, 0.04045, 0.04045),
        srgb_conversion_srgb_triplet_10_50_90: (0.01002283, 0.21404114, 0.78741229) -> (0.1, 0.5, 0.9),
        srgb_conversion_srgb_triplet_25_33_66: (0.05087609, 0.08898153, 0.39312313) -> (0.25, 0.33, 0.66),
        srgb_conversion_srgb_triplet_02_75_80: (0.00154799, 0.52252155, 0.60382734) -> (0.02, 0.75, 0.8),
        srgb_conversion_srgb_triplet_mid_srgb_50: (0.21404114, 0.21404114, 0.21404114) -> (0.5, 0.5, 0.5),
    ];

    conversion_tests![
        u8 = 0:
        srgb_conversion_linear_black_u8: (0, 0, 0) -> (0, 0, 0),
        srgb_conversion_linear_white_u8: (255, 255, 255) -> (255, 255, 255),
        srgb_conversion_linear_half_u8: (128, 128, 128) -> (188, 188, 188),
        srgb_conversion_linear_toe_threshold_u8: (1, 1, 1) -> (13, 13, 13),
        srgb_conversion_linear_low_01_u8: (3, 3, 3) -> (28, 28, 28),
        srgb_conversion_linear_mixed_18_50_01_u8: (45, 127, 2) -> (117, 187, 22),
    ];
}
