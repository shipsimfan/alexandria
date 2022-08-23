use alexandria_common::{Vector3, Vector4};
use std::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign},
};

#[cfg(target_os = "windows")]
type MatrixType = alexandria_dx11::LHRowMajorMatrix;

#[cfg(target_os = "linux")]
type MatrixType = alexandria_opengl::RHColumnMajorMatrix;

#[derive(Clone, Copy)]
pub struct Matrix(MatrixType);

impl Matrix {
    pub fn zero() -> Self {
        Matrix(MatrixType::zero())
    }

    pub fn identity() -> Self {
        Matrix(MatrixType::identity())
    }

    pub fn look_at(position: Vector3, target: Vector3, up: Vector3) -> Self {
        Matrix(MatrixType::look_at(position, target, up))
    }

    pub fn scale(x: f32, y: f32, z: f32) -> Self {
        Matrix(MatrixType::scale(x, y, z))
    }

    pub fn translation(x: f32, y: f32, z: f32) -> Self {
        Matrix(MatrixType::translation(x, y, z))
    }

    pub fn rotation(x: f32, y: f32, z: f32) -> Self {
        Matrix(MatrixType::rotation(x, y, z))
    }

    pub fn rotation_x(angle: f32) -> Self {
        Matrix(MatrixType::rotation_x(angle))
    }

    pub fn rotation_y(angle: f32) -> Self {
        Matrix(MatrixType::rotation_y(angle))
    }

    pub fn rotation_z(angle: f32) -> Self {
        Matrix(MatrixType::rotation_z(angle))
    }

    pub fn orthographic(width: f32, height: f32, near: f32, far: f32) -> Self {
        Matrix(MatrixType::orthographic(width, height, near, far))
    }

    pub fn perspective(fovy: f32, aspect: f32, near: f32, far: f32) -> Self {
        Matrix(MatrixType::perspective(fovy, aspect, near, far))
    }

    pub fn get(&self, col: usize, row: usize) -> f32 {
        self.0.get(col, row)
    }

    pub fn set(&mut self, col: usize, row: usize, value: f32) {
        self.0.set(col, row, value)
    }
}

impl Add for Matrix {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Matrix(self.0.add(rhs.0))
    }
}

impl AddAssign for Matrix {
    fn add_assign(&mut self, rhs: Self) {
        self.0.add_assign(rhs.0)
    }
}

impl Sub for Matrix {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Matrix(self.0.sub(rhs.0))
    }
}

impl SubAssign for Matrix {
    fn sub_assign(&mut self, rhs: Self) {
        self.0.sub_assign(rhs.0)
    }
}

impl Mul for Matrix {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Matrix(self.0.mul(rhs.0))
    }
}

impl MulAssign for Matrix {
    fn mul_assign(&mut self, rhs: Self) {
        self.0.mul_assign(rhs.0)
    }
}

impl Mul<Vector4> for Matrix {
    type Output = Vector4;

    fn mul(self, rhs: Vector4) -> Self::Output {
        self.0.mul(rhs)
    }
}

impl From<[f32; 4 * 4]> for Matrix {
    fn from(inner: [f32; 4 * 4]) -> Self {
        Matrix(inner.into())
    }
}

impl Into<[f32; 4 * 4]> for Matrix {
    fn into(self) -> [f32; 4 * 4] {
        self.0.into()
    }
}

impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl Debug for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.0, f)
    }
}
