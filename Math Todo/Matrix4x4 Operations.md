# Matrix4x4 Operations for Game-Oriented Libraries

## Construction & Access

### Accessors
- `row(i) -> Vector4<T>` / `set_row(i, v)`
- `col(i) -> Vector4<T>` / `set_col(i, v)`
- `get(r, c) -> T` / `set(r, c, v)`
- `transpose()`
- `as_slice()`
- Indexing ([row][column])
- Iterating (rows, columns, values row-major, values column-major)

---

## Arithmetic

### Matrix–Matrix
- `M * N` (composition)
- `M + N`, `M - N`
- Assignment forms: `*=`, `+=`, `-=`

### Matrix–Scalar 
- `M + s`, `M - s`, `M * s`, `M / s`, `M % s`

### Matrix–Vector
- `Matrix4x4 * Vector4` (or `Vector4 * Matrix4x4` depending on convention)
- `transform_point(p: Vector3<T>) -> Vector3<T>` *(uses w=1)*
- `transform_vector(v: Vector3<T>) -> Vector3<T>` *(uses w=0)*
- `transform_normal(n: Vector3<T>) -> Vector3<T>` *(see below; usually needs inverse-transpose)*

> Prefer explicit `transform_point/vector/normal` helpers over raw `mul` to avoid
> w-component mistakes.

---

## Determinant & Inversion

- `determinant() -> T`
- `inverse() -> Matrix4x4<T>`
- `try_inverse() -> Option<Matrix4x4<T>>`
- `is_invertible(eps)`
- `inverse_transpose_3x3()`

---

## Common Transform Matrices

### Translation / Rotation / Scale
- `from_translation(t: Vector3<T>)`
- `from_scale(s: Vector3<T>)` / `from_uniform_scale(s: T)`
- `from_rotation(q: Quaternion<T>)`
- `from_trs(translation, rotation, scale)` 
- `to_trs()`

### View (Camera)
- `look_at(eye, target, up)`

### Projection
- `perspective(fovy, aspect, z_near, z_far)`
- `perspective_infinite(fovy, aspect, z_near)`
- `orthographic(left, right, bottom, top, z_near, z_far)`
- `frustum(left, right, bottom, top, z_near, z_far)`

> Also document depth range conventions if relevant:
> - OpenGL-style NDC z ∈ [-1, 1]
> - D3D/Vulkan-style NDC z ∈ [0, 1]

---

## Decomposition & Queries

### Extract Components (when valid)
- `translation() -> Vector3<T>`
- `basis_x/y/z() -> Vector3<T>`
- `scale() -> Vector3<T>` *(approx; assumes no shear)*
- `rotation() -> Quaternion<T>` *(assumes orthonormal basis)*

### Classification
- `is_affine(eps)` (bottom row/col matches affine form)
- `is_orthonormal(eps)` *(3×3 rotation part)*

---

## Utilities

### Rounding & Validation
- `is_finite()`
- `abs()`
- `approx_eq(other, eps)`

---

## Conversions

### To/From Related Types
- `to_mat3()` / `from_mat3()` *(embedding/extracting 3×3)*
