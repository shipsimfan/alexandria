# Matrix3x3 Operations for Game-Oriented Libraries

This document outlines recommended operations for a `Matrix3x3<T>` type used for
3D rotation/scale (and 2D affine transforms embedded in 3×3). The scalar type `T`
is typically `f32` (or `f64`).

> Recommendation: be explicit about **storage** (row-major vs column-major) and
> **multiplication convention** (row-vector vs column-vector). Most bugs come
> from mismatched conventions, not the math itself.

---

## Core Type

- `Matrix3x3<T>`: 3×3 matrix

### Layout & Convention (must document)
- Storage: row-major or column-major
- Multiplication convention:
  - Column vectors: `v' = M * v`
  - Row vectors: `v' = v * M`
- Composition order (what `A * B` means)

---

## Construction & Access

### Basic Construction
- `identity()` / `IDENTITY`
- `zero()` / `ZERO` *(optional)*
- `from_rows([Vector3<T>; 3])` or `from_cols([Vector3<T>; 3])`
- `from_array([[T; 3]; 3])` / `to_array()`
- `from_flat_array([T; 9])` / `to_flat_array()` *(layout-dependent)*

### Accessors
- `row(i) -> Vector3<T>` / `set_row(i, v)`
- `col(i) -> Vector3<T>` / `set_col(i, v)`
- `get(r, c) -> T` / `set(r, c, v)`
- `transpose()`

---

## Arithmetic

### Matrix–Matrix
- `M * N` (composition)
- `M + N`, `M - N` *(optional but common)*
- Assignment forms: `*=`, `+=`, `-=`

### Matrix–Scalar *(optional)*
- `M * s`, `M / s`

### Matrix–Vector
- `Matrix3x3 * Vector3` (or `Vector3 * Matrix3x3` depending on convention)
- `transform_vector(v: Vector3<T>) -> Vector3<T>` *(explicit helper)*

---

## Determinant & Inversion *(float primary)*

- `determinant() -> T`
- `inverse() -> Matrix3x3<T>` *(may return Option/Result if non-invertible)*
- `try_inverse() -> Option<Matrix3x3<T>>`
- `is_invertible(eps)` *(optional)*

---

## Common Uses & Constructors *(float only)*

### 3D Rotation / Scale
- `from_rotation(q: Quaternion<T>)`
- `from_axis_angle(axis_unit, angle)`
- `from_scale(s: Vector3<T>)` / `from_uniform_scale(s: T)`
- `from_rotation_scale(rotation, scale)` *(optional convenience)*

### 2D Affine (Homogeneous 2D) *(optional but very useful)*
If you support 2D transforms via 3×3 homogeneous matrices:
- `from_translation_2d(t: Vector2<T>)`
- `from_rotation_2d(angle)`
- `from_scale_2d(s: Vector2<T>)`
- `from_trs_2d(t, angle, s)`
- `transform_point_2d(p: Vector2<T>) -> Vector2<T>` *(implicit w=1)*
- `transform_vector_2d(v: Vector2<T>) -> Vector2<T>` *(implicit w=0)*

> This is one of the best reasons to have a dedicated `Matrix3x3` in a game math
> library: fast and compact 2D transforms.

---

## Normal Transform Helpers

- `inverse_transpose()` *(often used for transforming normals in 3D when the 3×3
  includes non-uniform scale)*

> In many engines: normals should be transformed by the inverse-transpose of the
> linear part of the model matrix.

---

## Decomposition & Queries *(optional)*

### Extract Components (when valid)
- `basis_x/y/z() -> Vector3<T>` *(axes; depends on convention)*
- `scale() -> Vector3<T>` *(assumes no shear; approximate)*
- `rotation() -> Quaternion<T>` *(assumes orthonormal basis)*

### Classification
- `is_orthonormal(eps)`
- `is_symmetric(eps)` *(optional; niche)*

---

## Utilities

### Rounding & Validation *(float only)*
- `is_finite()`
- `approx_eq(other, eps)`

---

## Conversions

### To/From 4×4
- `to_mat4()` (embed into a 4×4 with last row/col as identity)
- `from_mat4_linear_part(m4)` (extract the upper-left 3×3)

### Packing / Interop
- `to_column_major_f32_array()` / `to_row_major_f32_array()`
- `to_rows()` / `to_cols()`

> Provide explicit packing helpers for GPU APIs / shader constants rather than
> relying on internal layout.

---

## Trait Gating Strategy (Rust)

Suggested bounds:

- Basic ops: `T: Copy + Add + Sub + Mul`
- Determinant/inversion/rotation constructors: `T: Float`

Example:
```rust
impl<T: Float> Matrix3x3<T> {
    fn determinant(self) -> T;
    fn try_inverse(self) -> Option<Self>;
    fn from_rotation(q: Quaternion<T>) -> Self;
    fn transform_vector3(self, v: Vector3<T>) -> Vector3<T>;
}
