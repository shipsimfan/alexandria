# Vector Operations for Game-Oriented Libraries

---

## `Vector3<T>`-Specific Operations

### Misc
- Swizzling

---

## `Vector2<T>`-Specific Operations

### Misc
- Remove `project`, `reflect`, `refract`
- Tests
   - `angle_between()`
   - `signed_angle()`
   - `distance_squared()`
   - `distance()`
   - `length_squared()`
   - `length()`
   - `normalize()`
   - `normalized()`
   - `is_normalized()`
   - `lerp()`
   - `nlerp()`
   - `slerp()`
   - `smoothstep()`
- Swizzling

### 2D Geometry
- `perp_cw()`, `perp_ccw()`
- `angle()` (angle from x-axis)
- `rotate(angle)`

### Cross-like Scalar
- `cross(v, u) -> T` (2D “signed area” / z-component of 3D cross)

### Dimension conversions
- `Vector2 -> Vector3` (`extend(z)`)