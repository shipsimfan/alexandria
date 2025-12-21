# Vector Operations for Game-Oriented Libraries

## Operations for `Vector2`, `Vector3`, and `Vector4`

### Constants
- `ZERO`
- `ONE`
- Unit axes: `X`, `Y` (and `Z`, `W` where applicable) (alternatively `UP`/`DOWN`, `RIGHT`/`LEFT`, `FORWARD`/`BACKWARD`, `ANA`/`KATA`)
- (float-only) `NAN`, `INFINITY`, `NEG_INFINITY`

### Other
 - Indexing
 - Iterating
 - Swizzling

---

## Arithmetic (Component-wise)

### Vector-Vector
- `v + u`
- `v - u`
- `v * u` (Hadamard / component-wise)
- `v / u` / `v % u` (component-wise)

> Consider naming Hadamard explicitly (`hadamard_mul`) if you want to avoid confusion
> with dot/cross.

### Vector-Scalar
- `v * s`
- `v / s` / `v % s`
- `v + s` / `v - s`

### Unary
- `-v` (signed numeric only)

### Assignment forms
- `+=`, `-=`, `*=`, `/=`

---

## Common Utilities

### Length & Normalization
- `length()`
- `length_squared()`
- `distance(a, b)` / `distance_squared(a, b)`
- `normalize()` (returns unit vector)
- `try_normalize()` / `normalize_or_zero(eps)` (avoid NaNs)
- `is_normalized(eps)`

### Angles
 - `angle_between`
 - `signed_angle`

### Dot Product
- `dot(v, u)`

### Roots
- `sqrt`
- `cbrt`

### Min/Max & Clamp
- `max_v(v, u)` (component-wise w/ vector)
- `min_v(v, u)` (component-wise w/ vector)
- `max(v, s)` (component-wise w/ scalar)
- `min(v, s)` (component-wise w/ scalar)
- `clamp_v(v, min, max)` (component-wise w/ vector)
- `clamp(v, min, max)` (component-wise w/ scalar)
- `abs()` (signed only)

### Interpolation
- `lerp(a, b, t)`
- `smoothstep(a, b, t)`
- `nlerp(a, b, t)`
- `slerp(a, b, t)`

### Rounding
- `floor()`, `ceil()`, `round()`, `trunc()`, `fract()`

### Validation
- `is_finite()`
- `is_nan()`

### Trig
- `acos()`
- `acosh()`
- `asin()`
- `asinh()`
- `atan()`
- `atan2()`
- `atanh()`
- `cos()`
- `cosh()`
- `sin()`
- `sinh()`
- `tan()`
- `tanh()`

### Other
- `copysign()`
- `exp()`
- `exp2()`
- `ln()`
- `log2()`
- `log10()`
- `powf()`
- `powi()`
- `signum()`

---

## `Vector2<T>`-Specific Operations

### 2D Geometry
- `perp_cw()`, `perp_ccw()`
- `angle()` (angle from x-axis)
- `rotate(angle)`

### Cross-like Scalar
- `cross(v, u) -> T` (2D “signed area” / z-component of 3D cross)

---

## `Vector3<T>`-Specific Operations

### Cross Product
- `cross(v, u) -> Vector3<T>`

### 3D Geometry
- `reflect(v, normal_unit)` (requires unit normal)
- `refract(v, normal_unit, eta)` 
- `project_onto(v, onto)` / `reject_from(v, onto)`

---

### Dimension conversions
- `Vector2 -> Vector3` (`extend(z)`)
- `Vector3 -> Vector4` (`extend(w)`)