# Matrix4x4 Operations for Game-Oriented Libraries

---

## Common Transform Matrices

### Translation / Rotation / Scale
- `from_rotation(q: Quaternion<T>)`
- `from_trs(translation, rotation, scale)` 

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

## Decomposition & Queries

### Extract Components (when valid)
- `translation() -> Vector3<T>`
- `basis_x/y/z() -> Vector3<T>`
- `scale() -> Vector3<T>` *(approx; assumes no shear)*
- `euler_rotation() -> Vector3<T>` *(assumes orthonormal basis)*
- `rotation() -> Quaternion<T>` *(assumes orthonormal basis)*
- `into_trs()`

### Classification
- `is_affine(eps)` (bottom row/col matches affine form)
- `is_orthonormal(eps)` *(3×3 rotation part)*

---

## Matrix3x3 interactions

### To/From
- `to_mat3()`
- `from_mat3()`

### Other
- `inverse_transpose_3x3()`

### Transformations
- `transform_normal(n: Vector3<T>) -> Vector3<T>` *(see below; usually needs inverse-transpose)*
