use crate::{
    Vector4,
    number::{One, Sqrt, Zero},
};
use std::ops::{Add, Div, Mul};

impl<T: Add<Output = T> + Mul<Output = T> + Div<Output = T> + Clone + PartialEq + Zero + One + Sqrt>
    Vector4<T>
{
    /// Normalize the values in the this vector, making its length 1
    pub fn normalized(self) -> Self {
        let length_squared = self.clone().length_squared();
        if length_squared == T::ONE {
            return self;
        }
        if length_squared == T::ZERO {
            return Vector4::ZERO;
        }

        self / length_squared.sqrt()
    }
}

#[cfg(test)]
mod tests {
    use crate::Vector4f;

    macro_rules! normalized_tests {
        [$(
            $test_name: ident:
                ($ix: literal, $iy: literal, $iz: literal, $iw: literal)
                -> ($ox: literal, $oy: literal, $oz: literal, $ow: literal),
        )*] => {$(
            #[test]
            fn $test_name() {
                const INPUT: Vector4f = Vector4f::new($ix, $iy, $iz, $iw);
                const OUTPUT: Vector4f = Vector4f::new($ox, $oy, $oz, $ow);

                let output = INPUT.normalized();

                assert!(output.approx_eq(OUTPUT, 1e-6), "normalized failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    normalized_tests![
        normalized_zero: (0.0, 0.0, 0.0, 0.0) -> (0.0, 0.0, 0.0, 0.0),
        normalized_unit_x: (1.0, 0.0, 0.0, 0.0) -> (1.0, 0.0, 0.0, 0.0),
        normalized_unit_w: (0.0, 0.0, 0.0, 1.0) -> (0.0, 0.0, 0.0, 1.0),
        normalized_simple_3_4: (3.0, 4.0, 0.0, 0.0) -> (0.6, 0.8, 0.0, 0.0),
        normalized_axis_z: (0.0, 0.0, 5.0, 0.0) -> (0.0, 0.0, 1.0, 0.0),
        normalized_all_ones: (1.0, 1.0, 1.0, 1.0) -> (0.5, 0.5, 0.5, 0.5),
        normalized_neg_mix: (-2.0, 2.0, -1.0, 0.5) -> (-0.65759595, 0.65759595, -0.32879797, 0.16439899),
        normalized_small: (0.001, -0.002, 0.003, -0.004) -> (0.18257419, -0.36514837, 0.54772256, -0.73029674),
        normalized_large_safe: (1000.0, -2000.0, 3000.0, -4000.0) -> (0.18257419, -0.36514837, 0.54772256, -0.73029674),
        normalized_mixed_int: (2.0, 3.0, 6.0, 7.0) -> (0.20203051, 0.30304575, 0.6060915, 0.70710677),
    ];
}
