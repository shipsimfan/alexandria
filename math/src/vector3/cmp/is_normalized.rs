use crate::{
    Vector3,
    number::{ApproxEq, One},
};
use std::{
    marker::Destruct,
    ops::{Add, Mul},
};

impl<T> Vector3<T> {
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
    use crate::Vector3f;

    macro_rules! length_squared_tests {
        [$(
            $test_name: ident: ($ix: literal, $iy: literal, $iz: literal) -> $o: literal,
        )*] => {$(
            #[test]
            fn $test_name() {
                const INPUT: Vector3f = Vector3f::new($ix, $iy, $iz);

                println!("TESTING: {}", INPUT.length_squared());

                assert_eq!(INPUT.is_normalized(0.001), $o);
            }
        )*};
    }

    length_squared_tests![
        is_normalized_unit_x: (1.0, 0.0, 0.0) -> true,
        is_normalized_unit_y: (0.0, 1.0, 0.0) -> true,
        is_normalized_unit_z: (0.0, 0.0, 1.0) -> true,
        is_normalized_unit_neg_x: (-1.0, 0.0, 0.0) -> true,
        is_normalized_unit_neg_y: (0.0, -1.0, 0.0) -> true,
        is_normalized_unit_neg_z: (0.0, 0.0, -1.0) -> true,

        is_normalized_zero_vector: (0.0, 0.0, 0.0) -> false,
        is_normalized_longer_than_one_axis: (2.0, 0.0, 0.0) -> false,
        is_normalized_shorter_than_one_axis: (0.5, 0.0, 0.0) -> false,

        is_normalized_pythagorean_3_4_5_pos: (0.6, 0.8, 0.0) -> true,
        is_normalized_pythagorean_3_4_5_mixed_sign: (-0.6, 0.8, 0.0) -> true,
        is_normalized_pythagorean_3_4_5_three_dim: (0.6, 0.0, 0.8) -> true,

        is_normalized_close_but_short: (0.99, 0.0, 0.0) -> false,
        is_normalized_close_but_long: (1.01, 0.0, 0.0) -> false,

        is_normalized_clearly_not_unit_diagonal: (0.5, 0.5, 0.5) -> false,
        is_normalized_non_unit_mixed: (1.0, 1.0, 0.0) -> false,
    ];
}
