# Matrix3x3 Operations for Game-Oriented Libraries

---

## Common Uses & Constructors

### 3D Rotation / Scale
- `from_rotation_scale(rotation, scale)`

### 2D Affine (Homogeneous 2D)
If you support 2D transforms via 3×3 homogeneous matrices:
- `from_translation_2d(t: Vector2<T>)`
- `from_rotation_2d(angle)`
- `from_scale_2d(s: Vector2<T>)`
- `from_trs_2d(t, angle, s)`
- `transform_point_2d(p: Vector2<T>) -> Vector2<T>` *(implicit w=1)*
- `transform_vector_2d(v: Vector2<T>) -> Vector2<T>` *(implicit w=0)*

---

## Normal Transform Helpers

- `inverse_transpose()` *(often used for transforming normals in 3D when the 3×3
  includes non-uniform scale)*

---

## Decomposition & Queries

### Classification
- `is_orthonormal(eps)`
- `is_symmetric(eps)`

---

## Conversions

### To/From 4×4
- `to_mat4()` (embed into a 4×4 with last row/col as identity)
- `from_mat4_linear_part(m4)` (extract the upper-left 3×3)

