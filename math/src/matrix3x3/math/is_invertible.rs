use crate::{
    Matrix3x3,
    number::{Abs, ApproxEq, FromF32, One, Zero},
};
use std::{
    marker::Destruct,
    ops::{DivAssign, Mul, SubAssign},
};

impl<T: Zero + One> Matrix3x3<T> {
    /// Is this matrix invertible?
    pub const fn is_invertible(self) -> bool
    where
        T: [const] Mul<Output = T>
            + [const] SubAssign
            + [const] DivAssign
            + [const] Abs
            + [const] ApproxEq
            + [const] FromF32
            + [const] Clone
            + [const] PartialOrd
            + [const] Destruct,
        T::Epsilon: [const] FromF32,
    {
        self.lu_decompose().is_some()
    }
}

#[cfg(test)]
mod tests {
    use crate::Matrix3x3f;

    macro_rules! is_invertible_tests {
        [$(
            $test_name: ident: ($i: expr) -> $o: literal,
        )*] => {$(
            #[test]
            fn $test_name() {
                const INPUT: Matrix3x3f = Matrix3x3f::from_row_array($i);
                const OUTPUT: bool = $o;

                assert_eq!(INPUT.is_invertible(), OUTPUT);
            }
        )*};
    }

    is_invertible_tests![];
}
