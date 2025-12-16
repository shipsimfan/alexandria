## Operations for Both `Color3` and `Color4`

### Clamping & Validation
- `clamp(min, max)`
- `saturate()` (float normalized only)
- `is_finite()` (float only)

### Interpolation
- `lerp(a, b, t)` — **only for linear-light spaces**
- Prefer explicit naming like `lerp_linear`

---

## Arithmetic Operations

### Safe / Component-wise
- `color + color`
- `color - color`
- `color * scalar`
- `color / scalar`

> Recommended: gate `Add`/`Sub` to linear-light spaces or expose as
> `add_components`, `mul_components`.

### Color Modulation
- `color * color` (component-wise tinting)
- Prefer named method: `modulate(other)`

---

## `Color4`-Specific Operations (Alpha)

### Alpha Manipulation
- `with_alpha(a)`
- `set_alpha(a)`
- `mul_alpha(k)`

### Premultiplied Alpha  
*(Linear + float only)*
- `premultiply()`
- `unpremultiply()`

### Compositing  
*(Linear + float only)*
- `over(background)`
- `under(foreground)`

---

## Color-Space-Sensitive Operations

### Color Space Conversion
- `to_linear()`
- `to_srgb()`
- `convert::<OtherSpace>()`

### Luminance
- `luminance_rec709()` (linear only)
- `luminance(coeffs)`

### Perceptual / Artistic Adjustments
- `exposure(ev)`
- `tone_map_*()`
- `lighten()`, `darken()` (space-specific, explicit naming)

---

## Integer (`u8`) and Packed Formats

- `from_rgba8(r, g, b, a)`
- `to_rgba8()`
- `from_argb32(u32)`
- `to_argb32()`
- `from_unorm8()`
- `to_unorm8()`

---

## Trait Gating Strategy (Rust)

Suggested marker traits:

- `LinearLight`
- `FloatChannel`
- `Normalized`

Example:

```rust
impl<Space: LinearLight> Color3<Space, f32> {
    fn lerp(self, other: Self, t: f32) -> Self;
}

impl<Space: LinearLight> Color4<Space, f32> {
    fn over(self, bg: Self) -> Self;
}
