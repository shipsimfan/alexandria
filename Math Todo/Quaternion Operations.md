# Quaternion Operations for Game-Oriented Libraries

---

## Construction & Access

### From Rotation Representations
- `from_axis_angle(axis_unit, angle)` *(implement with rotate(), test with it)*
- `from_euler(x, y, z)` 
- `from_rotation_matrix(mat3 | mat4)`
- `from_two_vectors(from_unit, to_unit)` *(shortest arc)*

---

## Core Operations

### Identity & Inversion
- `is_identity(eps)`

### Normalization 
- `normalize()`
- `is_normalized(eps)`

---

## Applying Rotations

### Rotate Vectors
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