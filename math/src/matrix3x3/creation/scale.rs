use crate::{
    Matrix3x3, Vector3,
    number::{One, Zero},
};
use std::marker::Destruct;

impl<T: Zero + One> Matrix3x3<T> {
    /// Make a [`Matrix3x3`] representing `scale`
    pub const fn from_scale(scale: Vector3<T>) -> Matrix3x3<T>
    where
        T: [const] Destruct,
    {
        Matrix3x3::diagonal(scale.x, scale.y, scale.z)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Matrix3x3f, Vector3f};

    macro_rules! scale_tests {
        [$(
            $test_name: ident:
                ([$ix: literal, $iy: literal, $iz: literal],
                [$sx: literal, $sy: literal, $sz: literal])
                -> [$ox: literal, $oy: literal, $oz: literal],
        )*] => {$(
            #[test]
            fn $test_name() {
                const INPUT: Vector3f = Vector3f::new($ix, $iy, $iz);
                const SCALE: Vector3f = Vector3f::new($sx, $sy, $sz);
                const OUTPUT: Vector3f = Vector3f::new($ox, $oy, $oz);

                let scale = Matrix3x3f::from_scale(SCALE);

                let output = scale.transform_point(INPUT);

                assert!(output.approx_eq(OUTPUT, 1e-6), "scale failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    scale_tests![
        scale_identity: ([1.0, -2.0, 3.5], [1.0, 1.0, 1.0]) -> [1.0, -2.0, 3.5],
        scale_zero: ([1.25, -2.5, 3.75], [0.0, 0.0, 0.0]) -> [0.0, 0.0, 0.0],
        scale_mixed_signs: ([2.0, -3.0, 4.0], [-1.0, 2.0, -0.5]) -> [-2.0, -6.0, -2.0],
        scale_fractional: ([0.5, -1.25, 2.0], [0.25, 0.5, 0.125]) -> [0.125, -0.625, 0.25],
        scale_by_powers_of_two: ([3.0, -5.5, 7.25], [2.0, 0.5, 4.0]) -> [6.0, -2.75, 29.0],
        scale_negative_zero: ([-0.0, 1.0, -2.0], [3.0, -4.0, 5.0]) -> [-0.0, -4.0, -10.0],
        scale_with_zero_components: ([9.0, 0.0, -3.0], [0.5, -7.0, 0.0]) -> [4.5, -0.0, -0.0],
        scale_small_values: ([0.0001, -0.0002, 0.0003], [1000.0, -2000.0, 3000.0]) -> [0.1, 0.4, 0.9],
        scale_large_but_finite: ([1.0e10, -2.0e10, 3.0e10], [2.0, 0.5, -1.0]) -> [2.0e10, -1.0e10, -3.0e10],
        scale_nonuniform: ([8.0, -6.0, 5.0], [1.5, -2.0, 0.25]) -> [12.0, 12.0, 1.25],
        scale_commutativity_check_a: ([2.5, -4.0, 6.0], [3.0, 0.25, -2.0]) -> [7.5, -1.0, -12.0],
        scale_commutativity_check_b: ([3.0, 0.25, -2.0], [2.5, -4.0, 6.0]) -> [7.5, -1.0, -12.0],
        scale_all_negative: ([-1.0, -2.0, -3.0], [-4.0, -0.5, -2.0]) -> [4.0, 1.0, 6.0],
        scale_with_one_axis_zero: ([7.0, -8.0, 9.0], [1.0, 0.0, 1.0]) -> [7.0, -0.0, 9.0],
        scale_decimal_halves: ([10.5, -3.5, 0.5], [0.5, 2.0, -4.0]) -> [5.25, -7.0, -2.0],
    ];
}
