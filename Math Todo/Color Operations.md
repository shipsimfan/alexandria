
### Constants
- `WHITE`
- `BLACK`
- Primary colors: `RED`, `GREEN`, and `BLUE`

### (De)serialization
 - Feature gate

### With functions
 - `with_r`, `with_g`, `with_b`

---

## Color-Space-Sensitive Operations

### Color Space Conversion
- `to_linear()`
- `to_srgb()`
- `convert::<OtherSpace>()`
- `convert_unchanged::<OtherSpace>()`

### Luminance
- `luminance_rec709()` (linear only)
- `luminance(coeffs)`

### Perceptual / Artistic Adjustments
- `exposure(ev)`            (`v' = v * (2 ^ ev)` for Linear)
- `tone_map_reinhard()`     (Linear)

---

## `Color4`-Specific Operations (Alpha)

### Alpha Manipulation
- `with_alpha(a)`

### Premultiplied Alpha  
*(Linear + float only)*
- `premultiply()`
- `unpremultiply()`

### Compositing  
*(Linear + float only)*
- `over(background)`
- `under(foreground)`

### Conversion
 - `rgb() -> Color3`

## `Color3`-Specific Operation
 - `with_alpha(a) -> Color4`
