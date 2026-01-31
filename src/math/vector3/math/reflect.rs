use crate::math::{
    Vector3,
    number::{FromF32, One, Sqrt, Zero},
};
use std::{
    marker::Destruct,
    ops::{Add, Div, Mul, Sub},
};

impl<T> Vector3<T> {
    /// Reflect this vector around `normal`, assuming `normal` is a unit vector
    pub const fn reflect_unit(self, normal: Self) -> Self
    where
        T: [const] Add<Output = T>
            + [const] Sub<Output = T>
            + [const] Mul<Output = T>
            + [const] Mul<Vector3<T>, Output = Vector3<T>>
            + [const] FromF32
            + [const] Clone
            + [const] Destruct,
    {
        self.clone() - T::from_f32(2.0) * self.dot(normal.clone()) * normal
    }
}

impl<
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Mul<Vector3<T>, Output = Vector3<T>>
        + Div<Output = T>
        + Sqrt
        + FromF32
        + Clone
        + PartialEq
        + Zero
        + One,
> Vector3<T>
{
    /// Reflect this vector around `normal`
    pub fn reflect(self, normal: Self) -> Self {
        self.reflect_unit(normal.normalized())
    }
}

#[cfg(test)]
mod tests {
    use crate::math::Vector3f;

    macro_rules! reflect_tests {
        [$(
            $test_name: ident:
                ($ix: literal, $iy: literal, $iz: literal),
                ($nx: literal, $ny: literal, $nz: literal)
                -> ($ox: literal, $oy: literal, $oz: literal),
        )*] => {$(
            #[test]
            fn $test_name() {
                const INPUT: Vector3f = Vector3f::new($ix, $iy, $iz);
                const NORMAL: Vector3f = Vector3f::new($nx, $ny, $nz);
                const OUTPUT: Vector3f = Vector3f::new($ox, $oy, $oz);

                let output = INPUT.reflect(NORMAL);

                assert!(output.approx_eq(OUTPUT, 1e-6), "reflect failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    reflect_tests![
        reflect_plane_y: (1.0, -2.0, 3.0), (0.0, 5.0, 0.0) -> (1.0, 2.0, 3.0),
        reflect_plane_x: (-4.0, 5.0, 2.0), (3.0, 0.0, 0.0) -> (4.0, 5.0, 2.0),
        reflect_plane_z: (2.0, 1.0, -7.0), (0.0, 0.0, -2.0) -> (2.0, 1.0, 7.0),

        reflect_perpendicular_scaled: (1.0, 0.0, 0.0), (0.0, 2.0, 0.0) -> (1.0, 0.0, 0.0),
        reflect_parallel_axis: (0.0, 3.0, 0.0), (0.0, 7.0, 0.0) -> (0.0, -3.0, 0.0),

        reflect_diag_xy_45: (1.0, 0.0, 0.0), (1.0, 1.0, 0.0) -> (0.0, -1.0, 0.0),
        reflect_diag_xy_swap: (0.0, 1.0, 0.0), (1.0, -1.0, 0.0) -> (1.0, 0.0, 0.0),
        reflect_parallel_diag: (2.0, 2.0, 0.0), (1.0, 1.0, 0.0) -> (-2.0, -2.0, 0.0),

        reflect_diag_xyz: (1.0, 2.0, 3.0), (1.0, 1.0, 1.0) -> (-3.0, -2.0, -1.0),
        reflect_in_plane_z: (1.0, 1.0, 0.0), (0.0, 0.0, 5.0) -> (1.0, 1.0, 0.0),

        reflect_normal_2_1_2: (2.0, -1.0, 4.0), (2.0, 1.0, 2.0) -> (-2.8888888, -3.4444444, -0.8888889),
        reflect_normal_1_2_2: (3.0, 0.0, 1.0), (1.0, 2.0, 2.0) -> (1.8888888, -2.2222223, -1.2222222),
        reflect_mixed_signs: (3.0, -4.0, 5.0), (1.0, 2.0, -2.0) -> (6.3333335, 2.6666667, -1.6666666),
    ];
}
