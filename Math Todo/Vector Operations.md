# Vector Operations for Game-Oriented Libraries

## Common Utilities

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

## `Vector2<T>` and `Vector3<T>` Operations

### Dimension conversions
- `Vector2 -> Vector3` (`extend(z)`)
- `Vector3 -> Vector4` (`extend(w)`)

### Angles
 - `signed_angle`