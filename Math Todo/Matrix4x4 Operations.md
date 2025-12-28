# Matrix4x4 Operations for Game-Oriented Libraries

### To/From
- `to_mat3()`
- `from_mat3()`

### Classification
- `is_orthonormal(eps)` *(3×3 rotation part)*
- `is_symmetric(eps)`

### Other
- `inverse_transpose_3x3()`

### Transformations
- `transform_normal(n: Vector3<T>) -> Vector3<T>` *(needs inverse-transpose)*

### Tests
- `from_trs()`