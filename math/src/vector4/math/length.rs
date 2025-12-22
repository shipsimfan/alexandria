use crate::{Vector4, number::Sqrt};
use std::ops::{Add, Mul};

impl<T: Add<Output = T> + Mul<Output = T> + Sqrt + Clone> Vector4<T> {
    /// Calculate the length of this vector
    pub fn length(self) -> T {
        self.length_squared().sqrt()
    }
}

#[cfg(test)]
mod tests {
    use crate::{Vector4f, number::ApproxEq};

    macro_rules! length_tests {
        [$(
            $test_name: ident: ($ix: literal, $iy: literal, $iz: literal, $iw: literal) -> $o: literal,
        )*] => {$(
            #[test]
            fn $test_name() {
                const INPUT: Vector4f = Vector4f::new($ix, $iy, $iz, $iw);
                const OUTPUT: f32 = $o;

                let output = INPUT.length();

                assert!(output.approx_eq(OUTPUT, 1e-6), "length failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    length_tests![
        length_zero: (0.0, 0.0, 0.0, 0.0) -> 0.0,
        length_unit_x: (1.0, 0.0, 0.0, 0.0) -> 1.0,
        length_unit_w: (0.0, 0.0, 0.0, 1.0) -> 1.0,

        length_all_ones: (1.0, 1.0, 1.0, 1.0) -> 2.0,
        length_all_halves: (0.5, 0.5, 0.5, 0.5) -> 1.0,
        length_quarter_x: (0.25, 0.0, 0.0, 0.0) -> 0.25,

        length_pythag_3_4: (3.0, 4.0, 0.0, 0.0) -> 5.0,
        length_pythag_6_8: (6.0, 8.0, 0.0, 0.0) -> 10.0,
        length_pythag_9_12: (9.0, 12.0, 0.0, 0.0) -> 15.0,
        length_pythag_negative: (-3.0, -4.0, 0.0, 0.0) -> 5.0,

        length_3d_1_2_2: (1.0, 2.0, 2.0, 0.0) -> 3.0,
        length_3d_mixed_sign: (-1.0, 2.0, -2.0, 0.0) -> 3.0,
        length_4d_square_25: (1.0, 2.0, 2.0, 4.0) -> 5.0,

        length_w_only_7: (0.0, 0.0, 0.0, 7.0) -> 7.0,
        length_large_pythag_3000_4000: (3000.0, 4000.0, 0.0, 0.0) -> 5000.0,
        length_small_scaled_square_25e_6: (0.001, 0.002, 0.002, 0.004) -> 0.005,

        length_huge_single_axis: (1e19, 0.0, 0.0, 0.0) -> 1e19,
        length_huge_with_tiny_components: (1e8, 3.0, 4.0, 0.0) -> 100000000.0,
    ];
}
