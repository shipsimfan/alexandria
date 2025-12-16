# Matrix4x4 Operations for Game-Oriented Libraries

This document outlines recommended operations for a `Matrix4x4<T>` type used for
3D transforms and projection in games. The scalar type `T` is typically `f32`
(or `f64`).

> Recommendation: be explicit about **storage** (row-major vs column-major) and
> **conventions** (row-vector vs column-vector multiplication). Many bugs come
> from mismatched conventions, not math mistakes.

---

## Core Type

- `Matrix4x4<T>`: 4×4 matrix

### Layout & Convention (must document)
- Storage: row-major or column-major
- Multiplication convention:
  - Column vectors: `v' = M * v`
  - Row vectors: `v' = v * M`
- Transform composition order (what `A * B` means)

---

## Construction & Access

### Basic Construction
- `identity()` / `IDENTITY`
- `zero()` / `ZERO` *(optional)*
- `from_rows([Vector4<T>; 4])` or `from_cols([Vector4<T>; 4])`
- `from_array([[T; 4]; 4])` / `to_array()`
- `from_flat_array([T; 16])` / `to_flat_array()` *(layout-dependent)*

### Accessors
- `row(i) -> Vector4<T>` / `set_row(i, v)`
- `col(i) -> Vector4<T>` / `set_col(i, v)`
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
- `Matrix4x4 * Vector4` (or `Vector4 * Matrix4x4` depending on convention)
- `transform_point(p: Vector3<T>) -> Vector3<T>` *(uses w=1)*
- `transform_vector(v: Vector3<T>) -> Vector3<T>` *(uses w=0)*
- `transform_normal(n: Vector3<T>) -> Vector3<T>` *(see below; usually needs inverse-transpose)*

> Prefer explicit `transform_point/vector/normal` helpers over raw `mul` to avoid
> w-component mistakes.

---

## Determinant & Inversion *(float primary)*

- `determinant() -> T`
- `inverse() -> Matrix4x4<T>` *(may return Option/Result if non-invertible)*
- `try_inverse() -> Option<Matrix4x4<T>>`
- `is_invertible(eps)` *(optional)*
- `inverse_transpose_3x3()` *(useful for normal transforms)*

---

## Common Transform Matrices *(float only)*

### Translation / Rotation / Scale
- `from_translation(t: Vector3<T>)`
- `from_scale(s: Vector3<T>)` / `from_uniform_scale(s: T)`
- `from_rotation(q: Quaternion<T>)`
- `from_trs(translation, rotation, scale)` *(very common)*
- `to_trs()` *(optional; decomposition is tricky with shear/non-uniform scale)*

### View (Camera)
- `look_at(eye, target, up)` *(right-handed or left-handed; be explicit)*
- `look_to(eye, forward, up)` *(optional)*

### Projection
- `perspective(fovy, aspect, z_near, z_far)`
- `perspective_infinite(fovy, aspect, z_near)` *(optional)*
- `orthographic(left, right, bottom, top, z_near, z_far)`
- `frustum(left, right, bottom, top, z_near, z_far)` *(optional)*

> Also document depth range conventions if relevant:
> - OpenGL-style NDC z ∈ [-1, 1]
> - D3D/Vulkan-style NDC z ∈ [0, 1]

---

## Decomposition & Queries *(optional)*

### Extract Components (when valid)
- `translation() -> Vector3<T>`
- `basis_x/y/z() -> Vector3<T>` *(axes; depends on convention)*
- `scale() -> Vector3<T>` *(approx; assumes no shear)*
- `rotation() -> Quaternion<T>` *(assumes orthonormal basis)*

### Classification
- `is_affine(eps)` (bottom row/col matches affine form)
- `is_orthonormal(eps)` *(3×3 rotation part)*

---

## Utilities

### Rounding & Validation *(float only)*
- `is_finite()`
- `abs()` *(optional; component-wise)*
- `approx_eq(other, eps)`

### Interpolation *(rare; optional)*
- Avoid lerping matrices directly for animation transforms.
- Prefer TRS interpolation (lerp translation/scale + slerp rotation).

---

## Conversions

### To/From Related Types
- `to_mat3()` / `from_mat3()` *(embedding/extracting 3×3)*
- `to_columns()` / `to_rows()`
- Interop packing:
  - `to_column_major_f32_array()` / `to_row_major_f32_array()` for GPU APIs

> Provide explicit named packing methods; don’t rely on “whatever the internal
> layout is” when talking to graphics APIs.

---

## Trait Gating Strategy (Rust)

Suggested bounds:

- Basic ops: `T: Copy + Add + Sub + Mul`
- Inversion/determinant/projection: `T: Float`

Example:
```rust
impl<T: Float> Matrix4x4<T> {
    fn determinant(self) -> T;
    fn try_inverse(self) -> Option<Self>;
    fn transform_point3(self, p: Vector3<T>) -> Vector3<T>;
    fn perspective(fovy: T, aspect: T, z_near: T, z_far: T) -> Self;
}
