use crate::{
    Vector4,
    number::{ApproxEq, One},
};
use std::{
    marker::Destruct,
    ops::{Add, Mul},
};

impl<T> Vector4<T> {
    /// Is this vector normalized (within `epsilon`)?
    pub const fn is_normalized(self, epsilon: T::Epsilon) -> bool
    where
        T: [const] Add<Output = T>
            + [const] Mul<Output = T>
            + [const] Clone
            + [const] ApproxEq
            + [const] Destruct
            + One,
        T::Epsilon: [const] Mul<Output = T::Epsilon> + [const] Clone + [const] Destruct,
    {
        self.length_squared()
            .approx_eq(T::ONE, epsilon.clone() * epsilon)
    }
}

#[cfg(test)]
mod tests {
    use crate::Vector4f;

    macro_rules! is_normalized_tests {
        [$(
            $test_name: ident: ($ix: literal, $iy: literal, $iz: literal, $iw: literal) -> $o: literal,
        )*] => {$(
            #[test]
            fn $test_name() {
                const INPUT: Vector4f = Vector4f::new($ix, $iy, $iz, $iw);

                assert_eq!(INPUT.is_normalized(0.001), $o);
            }
        )*};
    }

    is_normalized_tests![
        is_normalized_unit_x: (1.0, 0.0, 0.0, 0.0) -> true,
        is_normalized_unit_y: (0.0, 1.0, 0.0, 0.0) -> true,
        is_normalized_unit_z: (0.0, 0.0, 1.0, 0.0) -> true,
        is_normalized_unit_w: (0.0, 0.0, 0.0, 1.0) -> true,

        is_normalized_negative_unit_x: (-1.0, 0.0, 0.0, 0.0) -> true,
        is_normalized_all_negative_unit: (-0.5, -0.5, -0.5, -0.5) -> true,

        is_normalized_equal_components_half: (0.5, 0.5, 0.5, 0.5) -> true,
        is_normalized_two_components_inv_sqrt2_xy: (0.70710677, 0.70710677, 0.0, 0.0) -> true,
        is_normalized_two_components_inv_sqrt2_zw: (0.0, 0.0, 0.70710677, 0.70710677) -> true,

        is_normalized_three_components_inv_sqrt3_xyz: (0.57735026, 0.57735026, 0.57735026, 0.0) -> true,
        is_normalized_three_components_inv_sqrt3_yzw: (0.0, 0.57735026, 0.57735026, 0.57735026) -> true,

        is_normalized_mixed_sign_unit: (-0.70710677, 0.0, 0.70710677, 0.0) -> true,
        is_normalized_mixed_sign_four_way: (-0.5, 0.5, -0.5, 0.5) -> true,

        is_normalized_zero_vector: (0.0, 0.0, 0.0, 0.0) -> false,
        is_normalized_small_vector: (0.1, 0.0, 0.0, 0.0) -> false,
        is_normalized_len_gt_1_axis: (2.0, 0.0, 0.0, 0.0) -> false,
        is_normalized_len_gt_1_two_axes: (1.0, 1.0, 0.0, 0.0) -> false,
        is_normalized_len_lt_1_two_axes: (0.25, 0.25, 0.0, 0.0) -> false,

        is_normalized_near_unit_slightly_low: (0.9999, 0.0, 0.0, 0.0) -> false,
        is_normalized_near_unit_slightly_high: (1.0001, 0.0, 0.0, 0.0) -> false,

        is_normalized_large_components_not_unit: (1000.0, 0.0, 0.0, 0.0) -> false,
        is_normalized_typical_non_unit: (0.3, 0.4, 0.0, 0.0) -> false,
        is_normalized_randomish_non_unit: (0.2, -0.1, 0.05, 0.02) -> false,
    ];
}
