use crate::{
    Vector2,
    number::{ApproxEq, One},
};
use std::{
    marker::Destruct,
    ops::{Add, Mul},
};

impl<T> Vector2<T> {
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
    use crate::Vector2f;

    macro_rules! is_normalized_tests {
        [$(
            $test_name: ident: ($ix: literal, $iy: literal) -> $o: literal,
        )*] => {$(
            #[test]
            fn $test_name() {
                const INPUT: Vector2f = Vector2f::new($ix, $iy);

                assert_eq!(INPUT.is_normalized(0.001), $o);
            }
        )*};
    }

    is_normalized_tests![
        is_normalized_unit_x: (1.0, 0.0)-> true,
        is_normalized_unit_y: (0.0, 1.0)-> true,
        is_normalized_neg_unit_x: (-1.0, 0.0)-> true,
        is_normalized_neg_unit_y: (0.0, -1.0)-> true,

        is_normalized_diag_inv_sqrt2: (0.70710677, 0.70710677)-> true,
        is_normalized_three_four_five_scaled: (0.6, 0.8)-> true,
        is_normalized_three_four_five_scaled_swapped: (0.8, 0.6)-> true,
        is_normalized_three_four_five_scaled_mixed_sign: (-0.6, 0.8)-> true,

        is_normalized_zero: (0.0, 0.0)-> false,
        is_normalized_too_long_x2: (2.0, 0.0)-> false,
        is_normalized_too_long_y2: (0.0, -2.0)-> false,
        is_normalized_too_short_half_len: (0.3, 0.4)-> false,
        is_normalized_diag_too_long: (0.8, 0.8)-> false,

        is_normalized_almost_unit_short: (0.999, 0.0)-> false,
        is_normalized_almost_unit_long: (1.001, 0.0)-> false,
        is_normalized_diag_slightly_long: (0.7072, 0.7072)-> false,

        is_normalized_tiny_components: (1.0e-20, 0.0)-> false,
        is_normalized_large_components: (1.0e10, 0.0)-> false,
    ];
}
