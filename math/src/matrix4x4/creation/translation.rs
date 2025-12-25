use crate::{
    Matrix4x4, Vector3, Vector4,
    number::{One, Zero},
};
use std::marker::Destruct;

impl<T: Zero + One> Matrix4x4<T> {
    /// Create a matrix representing `translation`
    pub const fn from_translation(translation: Vector3<T>) -> Matrix4x4<T>
    where
        T: [const] Destruct,
    {
        Matrix4x4::new_rows(
            Vector4::new(T::ONE, T::ZERO, T::ZERO, translation.x),
            Vector4::new(T::ZERO, T::ONE, T::ZERO, translation.y),
            Vector4::new(T::ZERO, T::ZERO, T::ONE, translation.z),
            Vector4::new(T::ZERO, T::ZERO, T::ZERO, T::ONE),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::{Matrix4x4f, Vector3f};

    macro_rules! translation_tests {
        [$(
            $test_name: ident:
                ([$ix: literal, $iy: literal, $iz: literal],
                [$tx: literal, $ty: literal, $tz: literal])
                -> [$ox: literal, $oy: literal, $oz: literal],
        )*] => {$(
            #[test]
            fn $test_name() {
                const INPUT: Vector3f = Vector3f::new($ix, $iy, $iz);
                const TRANSLATION: Vector3f = Vector3f::new($tx, $ty, $tz);
                const OUTPUT: Vector3f = Vector3f::new($ox, $oy, $oz);

                let translation = Matrix4x4f::from_translation(TRANSLATION);

                let output = translation.transform_point(INPUT);

                assert!(output.approx_eq(OUTPUT, 1e-6), "translation failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    translation_tests![
        translate_zero_by_zero: ([0.0, 0.0, 0.0], [0.0, 0.0, 0.0]) -> [0.0, 0.0, 0.0],
        translate_origin_by_unit_axes: ([0.0, 0.0, 0.0], [1.0, -1.0, 2.0]) -> [1.0, -1.0, 2.0],
        translate_unit_axes_by_origin: ([1.0, -1.0, 2.0], [0.0, 0.0, 0.0]) -> [1.0, -1.0, 2.0],

        translate_positive_integers: ([1.0, 2.0, 3.0], [4.0, 5.0, 6.0]) -> [5.0, 7.0, 9.0],
        translate_mixed_signs: ([10.0, -2.0, 0.5], [-3.0, 4.0, -0.25]) -> [7.0, 2.0, 0.25],
        translate_all_negative: ([-1.0, -2.0, -3.0], [-4.0, -5.0, -6.0]) -> [-5.0, -7.0, -9.0],
        translate_cancel_to_zero: ([1.5, -2.5, 3.25], [-1.5, 2.5, -3.25]) -> [0.0, 0.0, 0.0],

        translate_fractional_simple: ([0.25, 0.5, 0.75], [0.25, 0.5, 0.25]) -> [0.5, 1.0, 1.0],
        translate_fractional_mixed: ([-0.125, 2.75, -3.5], [0.5, -0.75, 1.25]) -> [0.375, 2.0, -2.25],

        translate_large_magnitudes: ([1.0e20, -1.0e20, 3.5e19], [2.0e20, 1.0e20, -3.5e19]) -> [3.0e20, 0.0, 0.0],
        translate_small_magnitudes: ([1.0e-6, -2.0e-6, 3.0e-6], [4.0e-6, 5.0e-6, -6.0e-6]) -> [5.0e-6, 3.0e-6, -3.0e-6],

        translate_negative_zero_behavior: ([-0.0, 0.0, -0.0], [0.0, -0.0, 0.0]) -> [0.0, 0.0, 0.0],
        translate_identity_like: ([42.0, -17.0, 0.125], [0.0, 0.0, 0.0]) -> [42.0, -17.0, 0.125],

        translate_commutativity_check_a: ([3.0, -4.0, 5.0], [7.0, 8.0, -9.0]) -> [10.0, 4.0, -4.0],
        translate_commutativity_check_b: ([7.0, 8.0, -9.0], [3.0, -4.0, 5.0]) -> [10.0, 4.0, -4.0],

        translate_chain_like_step1: ([1.0, 1.0, 1.0], [2.0, 3.0, 4.0]) -> [3.0, 4.0, 5.0],
        translate_chain_like_step2: ([3.0, 4.0, 5.0], [-1.0, -2.0, -3.0]) -> [2.0, 2.0, 2.0],
    ];
}
