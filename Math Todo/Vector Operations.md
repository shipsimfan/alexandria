# Vector Operations for Game-Oriented Libraries

## Common Utilities

### Length & Normalization
- `length()`
- `length_squared()`
- `distance(a, b)` / `distance_squared(a, b)`
- `normalize()` (returns unit vector)
- `is_normalized(eps)`

### Angles
 - `angle_between`
 - `signed_angle`

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
- `recip()`

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