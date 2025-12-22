use crate::Vector4;
use std::{
    marker::Destruct,
    ops::{Add, Mul},
};

impl<T> Vector4<T> {
    /// Calculate the squared length of this vector
    pub const fn length_squared(self) -> T
    where
        T: [const] Add<Output = T> + [const] Mul<Output = T> + [const] Clone + [const] Destruct,
    {
        self.clone().dot(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Vector4f, number::ApproxEq};

    macro_rules! length_squared_tests {
        [$(
            $test_name: ident: ($ix: literal, $iy: literal, $iz: literal, $iw: literal) -> $o: literal,
        )*] => {$(
            #[test]
            fn $test_name() {
                const INPUT: Vector4f = Vector4f::new($ix, $iy, $iz, $iw);
                const OUTPUT: f32 = $o;

                let output = INPUT.length_squared();

                assert!(output.approx_eq(OUTPUT, 1e-6), "length squared failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    length_squared_tests![
        length_squared_zero: (0.0, 0.0, 0.0, 0.0) -> 0.0,
        length_squared_unit_x: (1.0, 0.0, 0.0, 0.0) -> 1.0,
        length_squared_unit_y: (0.0, 1.0, 0.0, 0.0) -> 1.0,
        length_squared_unit_z: (0.0, 0.0, 1.0, 0.0) -> 1.0,
        length_squared_unit_w: (0.0, 0.0, 0.0, 1.0) -> 1.0,

        length_squared_neg_units: (-1.0, -1.0, -1.0, -1.0) -> 4.0,
        length_squared_mixed_signs: (-1.0, 2.0, -3.0, 4.0) -> 30.0,
        length_squared_pythag_3_4_12: (3.0, 4.0, 12.0, 0.0) -> 169.0,
        length_squared_all_twos: (2.0, 2.0, 2.0, 2.0) -> 16.0,
        length_squared_halves: (0.5, 0.5, 0.5, 0.5) -> 1.0,

        length_squared_neg_zeroes: (-0.0, -0.0, -0.0, -0.0) -> 0.0,
        length_squared_cancel_like_values: (1000.0, -1000.0, 1000.0, -1000.0) -> 4000000.0,

        length_squared_subnormal_min: (1.401298464e-45, 0.0, 0.0, 0.0) -> 0.0,
        length_squared_small_normals: (1.0e-10, -1.0e-10, 1.0e-10, -1.0e-10) -> 3.999999809e-20,

        length_squared_big_but_finite: (1.0e19, 1.0e19, 0.0, 0.0) -> 1.999999936e38,
    ];
}
