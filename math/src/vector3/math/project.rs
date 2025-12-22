use crate::Vector3;
use std::{
    marker::Destruct,
    ops::{Add, Div, Mul},
};

impl<T> Vector3<T> {
    /// Project this vector onto `normal`
    pub const fn project(self, normal: Self) -> Self
    where
        T: [const] Add<Output = T>
            + [const] Mul<Output = T>
            + [const] Div<Output = T>
            + [const] Clone
            + [const] Destruct,
    {
        normal.clone() * (normal.clone().dot(self) / normal.length_squared())
    }
}

#[cfg(test)]
mod tests {
    use crate::Vector3f;

    macro_rules! project_tests {
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

                let output = INPUT.project(NORMAL);

                assert!(output.approx_eq(OUTPUT, 1e-6), "project failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    project_tests![
        project_axis_x_nonunit: (3.5, -2.0, 1.25), (2.0, 0.0, 0.0) -> (3.5, 0.0, 0.0),

        project_axis_y_negative_normal: (1.0, -4.0, 2.0), (0.0, -2.0, 0.0) -> (0.0, -4.0, 0.0),

        project_axis_y_unit: (1.0, 2.0, 3.0), (0.0, 1.0, 0.0) -> (0.0, 2.0, 0.0),

        project_diag_xy_v_along_x: (2.0, 0.0, 0.0), (1.0, 1.0, 0.0) -> (1.0, 1.0, 0.0),

        project_diag_xy_general: (1.0, 3.0, 0.0), (1.0, 1.0, 0.0) -> (2.0, 2.0, 0.0),

        project_axis_y_from_yz_vector: (0.0, 2.0, 2.0), (0.0, 2.0, 0.0) -> (0.0, 2.0, 0.0),

        project_yz_nonunit_from_pure_y: (0.0, 4.0, 0.0), (0.0, 2.0, 2.0) -> (0.0, 2.0, 2.0),

        project_yz_nonunit_parallel: (0.0, 4.0, 4.0), (0.0, 2.0, 2.0) -> (0.0, 4.0, 4.0),

        project_yz_nonunit_ignores_x: (2.0, 4.0, 0.0), (0.0, 2.0, 2.0) -> (0.0, 2.0, 2.0),

        project_nonunit_returns_self: (1.0, 1.0, 0.0), (2.0, 2.0, 0.0) -> (1.0, 1.0, 0.0),

        project_nonunit_mixed_signs: (3.0, -1.0, 0.0), (2.0, 2.0, 0.0) -> (1.0, 1.0, 0.0),
    ];
}
