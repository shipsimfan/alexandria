# Quaternion Operations for Game-Oriented Libraries

This document outlines recommended operations for a `Quaternion<T>` type used
primarily for **3D rotations** in games. The scalar type `T` is typically `f32`
(or `f64`), and most operations assume **unit quaternions** unless stated
otherwise.

> Recommendation: treat quaternions as a **rotation type**, not a general
> four-dimensional math vector.

---

## Core Type

- `Quaternion<T>`: `(x, y, z, w)`
  - Vector part `(x, y, z)`
  - Scalar part `w`

---

## Construction & Access

### Basic Construction
- `new(x, y, z, w)`
- `identity()` / `IDENTITY`
- `from_array([T; 4])`
- `to_array()`
- `from_tuple((T, T, T, T))`
- `to_tuple()`
- `from_slice(&[T])`
- `as_slice()`

### From Rotation Representations
- `from_axis_angle(axis_unit, angle)`
- `from_euler(x, y, z)` 
- `from_rotation_matrix(mat3 | mat4)`
- `from_two_vectors(from_unit, to_unit)` *(shortest arc)*

### Accessors
- `vector()` → `Vector3<T>`
- `scalar()` → `T`
- Iterator
- Indexing

---

## Core Operations

### Identity & Inversion
- `conjugate()`
- `inverse()`
- `is_identity(eps)`

### Normalization 
- `length()`
- `length_squared()`
- `normalize()`
- `try_normalize()` / `normalize_or_identity(eps)`
- `is_normalized(eps)`

---

## Quaternion Arithmetic

### Quaternion–Quaternion
- `q * r` (composition: apply `r`, then `q`)
- `q *= r`

> Clearly document multiplication order and convention.

### Quaternion–Scalar *(optional)*
- `q * s`
- `q / s`

### Unary
- `-q` (represents the same rotation as `q`)

---

## Applying Rotations

### Rotate Vectors
- `rotate_vector(v: Vector3<T>) -> Vector3<T>`
- `impl Mul<Vector3<T>> for Quaternion<T>`

### Basis / Axes
- `forward()`, `right()`, `up()` 
- and `basis_x()`, `basis_y()`, `basis_z()`

---

## Interpolation

### Linear & Spherical *(unit quaternions assumed)*
- `lerp(q0, q1, t)` *(fast, not constant-speed; renormalize)*
- `nlerp(q0, q1, t)` *(normalized lerp; common in games)*
- `slerp(q0, q1, t)` *(constant angular velocity)*

> Ensure shortest-path handling (`dot < 0` → negate one quaternion).

---

## Decomposition & Queries

### Angle / Axis
- `to_axis_angle() -> (axis_unit, angle)`
- `angle()` *(magnitude of rotation)*

### Euler Angles
- `to_euler()`

> Euler conversions are inherently ambiguous; document conventions loudly.

### Comparison
- `approx_eq(other, eps)`
- `same_rotation(other, eps)` *(treats `q` and `-q` as equal)*

---

## Advanced Operations

### Relative Rotations
- `between(a, b)` *(equivalent to `b * a.inverse()`)*

### Integration *(physics / animation)*
- `integrate_angular_velocity(omega, dt)`
- `from_angular_velocity(omega, dt)`

---

## Conversions

### To/From Matrices
- `to_mat3()`
- `to_mat4()`
- `from_mat3()`
- `from_mat4()`