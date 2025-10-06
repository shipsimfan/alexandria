# ToDo
 1. Combine settings into single `GraphicsSettings` struct
 2. Add window control functions
    1. `quit`
    2. `set_position`
    3. `set_size`
    4. `set_refresh_rate`
    5. `set_vsync`
    6. `set_display_mode`
    7. `set_graphics_settings`
    8. `set_window_title`
 3. Add clearing of render target
    1. Add `SwapChainObjects` struct with relevant objects
    2. Add clearing of the render target view in `end_render`
    3. Add support for re-creation of `SwapChainObjects`
 4. Add proper `begin_render`, `end_render`, and `RenderContext` semantics
 5. Add shaders
 6. Add meshes
 7. Add mesh rendering
 8. Fully implement triangle example
 9. Add constant buffers
 10. Add input system
 11. Add keyboard input support
 12. Add cube example with movement