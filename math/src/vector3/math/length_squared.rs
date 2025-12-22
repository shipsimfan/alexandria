use crate::Vector3;
use std::{
    marker::Destruct,
    ops::{Add, Mul},
};

impl<T> Vector3<T> {
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
    use crate::{Vector3f, number::ApproxEq};

    macro_rules! length_squared_tests {
        [$(
            $test_name: ident: ($ix: literal, $iy: literal, $iz: literal) -> $o: literal,
        )*] => {$(
            #[test]
            fn $test_name() {
                const INPUT: Vector3f = Vector3f::new($ix, $iy, $iz);
                const OUTPUT: f32 = $o;

                let output = INPUT.length_squared();

                assert!(output.approx_eq(OUTPUT, 1e-6), "length squared failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    length_squared_tests![
        length_squared_zero: (0.0, 0.0, 0.0) -> 0.0,
        length_squared_unit_x: (1.0, 0.0, 0.0) -> 1.0,
        length_squared_unit_y: (0.0, 1.0, 0.0) -> 1.0,
        length_squared_unit_z: (0.0, 0.0, 1.0) -> 1.0,
        length_squared_pythag_122: (1.0, 2.0, 2.0) -> 9.0,
        length_squared_pythag_neg_122: (-1.0, -2.0, -2.0) -> 9.0,
        length_squared_3_4_0: (3.0, -4.0, 0.0) -> 25.0,
        length_squared_half_x: (0.5, 0.0, 0.0) -> 0.25,
        length_squared_half_all: (0.5, 0.5, 0.5) -> 0.75,
        length_squared_quarter_half_one: (0.25, -0.5, 1.0) -> 1.3125,
        length_squared_two_half_quarter: (2.0, -0.5, 0.25) -> 4.3125,
        length_squared_powers_8_16_32: (8.0, 16.0, 32.0) -> 1344.0,
        length_squared_ten_zero_ten: (-10.0, 0.0, 10.0) -> 200.0,
        length_squared_mixed_halves: (1.5, 2.0, -2.5) -> 12.5,
        length_squared_big_powers: (1024.0, -512.0, 256.0) -> 1376256.0,
        length_squared_small_powers: (0.125, 0.25, 0.5) -> 0.328125,
    ];
}
