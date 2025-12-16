# Vector Operations for Game-Oriented Libraries

This document outlines recommended operations for `Vector2`, `Vector3`, and `Vector4`
types, parameterized over an **underlying scalar type** `T` (e.g. `i32`, `u32`, `f32`, `f64`).
The goal is to provide a complete, ergonomic set of math operations while keeping
semantics clear and avoiding ambiguous APIs.

---

## Core Types

- `Vector2<T>`: `(x, y)`
- `Vector3<T>`: `(x, y, z)`
- `Vector4<T>`: `(x, y, z, w)`

> Recommendation: keep `VectorN<T>` “pure math” types. Put engine-specific meaning
> (positions vs directions vs normals, SIMD packing, coordinate spaces) in wrappers.

---

## Operations for `Vector2`, `Vector3`, and `Vector4`

### Construction & Access
- `new(x, y[, z[, w]])`
- `from_array([T; N])`
- `to_array()`
- `splat(v)` (all components = `v`)
- Component accessors: `x()`, `y()`, `z()`, `w()`
- `set_x(v)`, etc. (or public fields, depending on style)
- `map(f)` / `map2(other, f)` (component-wise mapping)

### Constants
- `ZERO`
- `ONE`
- Unit axes: `X`, `Y` (and `Z`, `W` where applicable)
- (float-only) `NAN`, `INFINITY` constants are usually not worth adding; prefer `is_finite`

### Formatting & Debug
- `Debug`, `Display` (optional)
- `from_str` / parsing is optional and usually not needed in core math

---

## Arithmetic (Component-wise)

### Vector-Vector
- `v + u`
- `v - u`
- `v * u` (Hadamard / component-wise)
- `v / u` (component-wise) *(be cautious for ints / division by zero)*

> Consider naming Hadamard explicitly (`hadamard_mul`) if you want to avoid confusion
> with dot/cross.

### Vector-Scalar
- `v * s`
- `v / s`
- `v + s` / `v - s` *(optional; can be convenient but sometimes surprising)*

### Unary
- `-v` (signed numeric only)

### Assignment forms
- `+=`, `-=`, `*=`, `/=`

---

## Common Utilities

### Length & Normalization *(float only)*
- `length()`
- `length_squared()`
- `distance(a, b)` / `distance_squared(a, b)`
- `normalize()` (returns unit vector)
- `try_normalize()` / `normalize_or_zero(eps)` (avoid NaNs)
- `is_normalized(eps)`

### Dot Product
- `dot(v, u)` *(works for ints too, but overflow concerns; float is the primary use)*

### Min/Max & Clamp
- `min(v, u)` (component-wise)
- `max(v, u)` (component-wise)
- `clamp(v, min, max)` (component-wise)
- `abs()` (signed only)

### Interpolation *(float only)*
- `lerp(a, b, t)`
- `smoothstep(a, b, t)` *(optional)*
- `nlerp(a, b, t)` (normalized lerp, useful for directions)

### Rounding *(float only)*
- `floor()`, `ceil()`, `round()`, `trunc()`, `fract()`

### Validation *(float only)*
- `is_finite()`
- `is_nan()` *(usually less useful than `is_finite`)*

---

## `Vector2<T>`-Specific Operations

### 2D Geometry *(float preferred)*
- `perp()` (e.g. `(−y, x)`)
- `angle()` (angle from x-axis) *(optional)*
- `rotate(angle)` *(optional, but handy in gameplay code)*

### Cross-like Scalar
- `cross(v, u) -> T` (2D “signed area” / z-component of 3D cross)

---

## `Vector3<T>`-Specific Operations

### Cross Product
- `cross(v, u) -> Vector3<T>`

### 3D Geometry *(float only)*
- `reflect(v, normal_unit)` (requires unit normal)
- `refract(v, normal_unit, eta)` *(optional)*
- `project_onto(v, onto)` / `reject_from(v, onto)` *(optional but useful)*

---

## `Vector4<T>`-Specific Operations

### Homogeneous / Packing Helpers *(optional, engine-dependent)*
- `xyz()` (drop `w`) -> `Vector3<T>`
- `extend(v3, w)` / `truncate()` naming varies

> Avoid hard-coding “position vs direction” meaning into `w` in the core math type.
> Provide helpers, but keep semantics in higher-level wrappers.

---

## Conversions & Casting

### Numeric casts
- `cast<U>()` / `as_*()` conversions (explicit, never implicit)
- `to_f32()`, `to_i32()` convenience methods if you like

### Dimension conversions
- `Vector2 -> Vector3` (`extend(z)`)
- `Vector3 -> Vector4` (`extend(w)`)
- `Vector4 -> Vector3` (`truncate()`)

---

## Trait Gating Strategy (Rust)

Suggested bounds (illustrative):

- For basic arithmetic: `T: Copy + Add + Sub + Mul + Div`
- For normalization/length: `T: Float` (your own float trait or `num_traits::Float`)
- For cross product (3D): `T: Copy + Sub + Mul`

Example:
```rust
impl<T: Float> Vector3<T> {
    fn length(self) -> T;
    fn normalize(self) -> Self;
    fn dot(self, other: Self) -> T;
}

impl<T: Copy + Sub<Output=T> + Mul<Output=T>> Vector3<T> {
    fn cross(self, other: Self) -> Self;
}
