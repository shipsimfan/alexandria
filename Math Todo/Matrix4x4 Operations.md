# Matrix4x4 Operations for Game-Oriented Libraries

## Arithmetic

### Matrix–Vector
- `Matrix4x4 * Vector4` (or `Vector4 * Matrix4x4` depending on convention)
- `transform_point(p: Vector3<T>) -> Vector3<T>` *(uses w=1)*
- `transform_vector(v: Vector3<T>) -> Vector3<T>` *(uses w=0)*
- `transform_normal(n: Vector3<T>) -> Vector3<T>` *(see below; usually needs inverse-transpose)*

### Matrix–Matrix
- `M * N` (composition)
- `M + N`, `M - N`
- Assignment forms: `*=`, `+=`, `-=`

---

## Common Transform Matrices

### Translation / Rotation / Scale
- `translation(t: Vector3<T>)`
- `scale(s: Vector3<T>)` / `uniform_scale(s: T)`
- `euler_rotation(r: Vector3<T>)`
- `rotation(q: Quaternion<T>)`
- `trs(translation, rotation, scale)` 

### Projection
- `perspective(fovy, aspect, z_near, z_far)`
- `perspective_infinite(fovy, aspect, z_near)`
- `orthographic(left, right, bottom, top, z_near, z_far)`
- `frustum(left, right, bottom, top, z_near, z_far)`

### View (Camera)
- `look_at(eye, target, up)`

> - Vulkan-style NDC z ∈ [0, 1]
> - Left-handed

---

## Determinant & Inversion

- `determinant() -> T`
- `inverse() -> Matrix4x4<T>`
- `try_inverse() -> Option<Matrix4x4<T>>`
- `is_invertible(eps)`
- `inverse_transpose_3x3()`

---

## Decomposition & Queries

### Extract Components (when valid)
- `translation() -> Vector3<T>`
- `basis_x/y/z() -> Vector3<T>`
- `scale() -> Vector3<T>` *(approx; assumes no shear)*
- `rotation() -> Quaternion<T>` *(assumes orthonormal basis)*
- `into_trs()`

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
