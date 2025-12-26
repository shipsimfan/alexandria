# Matrix4x4 Operations for Game-Oriented Libraries

---

## Quaternion interactions

### Extract Components (when valid)
- `rotation() -> Quaternion<T>` *(assumes orthonormal basis)*
- `into_trs() -> (Vector3<T>, Quaternion<T>, Vector3<T>)` *(assumes orthonormal basis and no shear)*

### Translation / Rotation / Scale
- `from_rotation(q: Quaternion<T>)`
- `from_trs(translation, rotation, scale)` 

---

## Matrix3x3 interactions

### To/From
- `to_mat3()`
- `from_mat3()`

### Classification
- `is_orthonormal(eps)` *(3×3 rotation part)*

### Other
- `inverse_transpose_3x3()`

### Transformations
- `transform_normal(n: Vector3<T>) -> Vector3<T>` *(needs inverse-transpose)*
