use crate::{Vector3, Vector4};
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Clone, Copy)]
pub struct Matrix([f32; 4 * 4]);

impl Matrix {
    pub const fn zero() -> Self {
        Matrix([0.0; 4 * 4])
    }

    pub fn identity() -> Self {
        let mut matrix = Matrix::zero();
        matrix.set(0, 0, 1.0);
        matrix.set(1, 1, 1.0);
        matrix.set(2, 2, 1.0);
        matrix.set(3, 3, 1.0);
        matrix
    }

    pub fn look_at(position: Vector3, target: Vector3, up: Vector3) -> Matrix {
        let z_axis = (target - position).normal();
        let x_axis = (up.cross(z_axis)).normal();
        let y_axis = z_axis.cross(x_axis);

        let mut matrix = Matrix::zero();
        matrix.set(0, 0, x_axis.x());
        matrix.set(0, 1, x_axis.y());
        matrix.set(0, 2, x_axis.z());
        matrix.set(0, 3, -x_axis.dot(position));
        matrix.set(1, 0, y_axis.x());
        matrix.set(1, 1, y_axis.y());
        matrix.set(1, 2, y_axis.z());
        matrix.set(1, 3, -y_axis.dot(position));
        matrix.set(2, 0, z_axis.x());
        matrix.set(2, 1, z_axis.y());
        matrix.set(2, 2, z_axis.z());
        matrix.set(2, 3, -z_axis.dot(position));
        matrix.set(3, 3, 1.0);
        matrix
    }

    pub fn scale(x: f32, y: f32, z: f32) -> Matrix {
        let mut matrix = Matrix::identity();
        matrix.set(0, 0, x);
        matrix.set(1, 1, y);
        matrix.set(2, 2, z);
        matrix
    }

    pub fn translation(x: f32, y: f32, z: f32) -> Matrix {
        let mut matrix = Matrix::identity();
        matrix.set(0, 3, x);
        matrix.set(1, 3, y);
        matrix.set(2, 3, z);
        matrix
    }

    pub fn rotation(x: f32, y: f32, z: f32) -> Matrix {
        Self::rotation_z(z) * Self::rotation_x(x) * Self::rotation_y(y)
    }

    pub fn rotation_x(θ: f32) -> Matrix {
        let mut matrix = Matrix::identity();

        let c = θ.cos();
        let s = θ.sin();

        matrix.set(1, 1, c);
        matrix.set(2, 1, -s);
        matrix.set(1, 2, s);
        matrix.set(2, 2, c);

        matrix
    }

    pub fn rotation_y(θ: f32) -> Matrix {
        let mut matrix = Matrix::identity();

        let c = θ.cos();
        let s = θ.sin();

        matrix.set(0, 0, c);
        matrix.set(2, 0, s);
        matrix.set(0, 2, -s);
        matrix.set(2, 2, c);

        matrix
    }

    pub fn rotation_z(θ: f32) -> Matrix {
        let mut matrix = Matrix::identity();

        let c = θ.cos();
        let s = θ.sin();

        matrix.set(0, 0, c);
        matrix.set(1, 0, -s);
        matrix.set(0, 1, s);
        matrix.set(1, 1, c);

        matrix
    }

    pub fn orthographic(width: f32, height: f32, near: f32, far: f32) -> Matrix {
        let mut matrix = Matrix::identity();
        matrix.set(0, 0, 2.0 / width);
        matrix.set(1, 1, 2.0 / height);
        matrix.set(2, 2, 1.0 / (far - near));
        matrix.set(2, 3, -near / (far - near));
        matrix
    }

    pub fn perspective(fovy: f32, aspect: f32, near: f32, far: f32) -> Matrix {
        let y_scale = 1.0 / (fovy / 2.0).tan();
        let x_scale = y_scale / aspect;

        let mut matrix = Matrix::zero();
        matrix.set(0, 0, x_scale);
        matrix.set(1, 1, y_scale);
        matrix.set(2, 2, far / (far - near));
        matrix.set(2, 3, -(near * far) / (far - near));
        matrix.set(3, 2, 1.0);
        matrix
    }

    pub fn get(&self, col: usize, row: usize) -> f32 {
        self.0[col * 4 + row]
    }

    pub fn set(&mut self, col: usize, row: usize, val: f32) {
        self.0[col * 4 + row] = val
    }
}

impl Add for Matrix {
    type Output = Matrix;

    fn add(mut self, rhs: Matrix) -> Matrix {
        for i in 0..4 {
            for j in 0..4 {
                self.set(i, j, self.get(i, j) + rhs.get(i, j))
            }
        }

        self
    }
}

impl AddAssign for Matrix {
    fn add_assign(&mut self, rhs: Matrix) {
        *self = *self + rhs;
    }
}

impl Sub for Matrix {
    type Output = Matrix;

    fn sub(mut self, rhs: Matrix) -> Matrix {
        for i in 0..4 {
            for j in 0..4 {
                self.set(i, j, self.get(i, j) - rhs.get(i, j))
            }
        }

        self
    }
}

impl SubAssign for Matrix {
    fn sub_assign(&mut self, rhs: Matrix) {
        *self = *self - rhs;
    }
}

impl Mul<Vector4> for Matrix {
    type Output = Vector4;

    fn mul(self, rhs: Vector4) -> Vector4 {
        Vector4::new(
            self.get(0, 0) * rhs.x()
                + self.get(1, 0) * rhs.y()
                + self.get(2, 0) * rhs.z()
                + self.get(3, 0) * rhs.w(),
            self.0[4] * rhs.x() + self.0[5] * rhs.y() + self.0[6] * rhs.z() + self.0[7] * rhs.w(),
            self.0[8] * rhs.x() + self.0[9] * rhs.y() + self.0[10] * rhs.z() + self.0[11] * rhs.w(),
            self.0[12] * rhs.x()
                + self.0[13] * rhs.y()
                + self.0[14] * rhs.z()
                + self.0[15] * rhs.w(),
        )
    }
}

impl Mul for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Matrix) -> Matrix {
        let mut ret = Matrix::zero();

        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    ret.set(i, j, ret.get(i, j) + self.get(i, k) * rhs.get(k, j));
                }
            }
        }

        ret
    }
}

impl MulAssign for Matrix {
    fn mul_assign(&mut self, rhs: Matrix) {
        *self = *self * rhs;
    }
}

impl MulAssign<Matrix> for Vector4 {
    fn mul_assign(&mut self, rhs: Matrix) {
        *self = rhs * *self;
    }
}

impl From<[f32; 4 * 4]> for Matrix {
    fn from(vals: [f32; 4 * 4]) -> Matrix {
        Matrix(vals)
    }
}

impl Into<[f32; 4 * 4]> for Matrix {
    fn into(self) -> [f32; 4 * 4] {
        self.0
    }
}

impl std::fmt::Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for i in 0..4 {
            writeln!(
                f,
                "| {: <5} {: <5} {: <5} {: <5} |",
                self.0[i * 4 + 0],
                self.0[i * 4 + 1],
                self.0[i * 4 + 2],
                self.0[i * 4 + 3]
            )?;
        }

        Ok(())
    }
}
