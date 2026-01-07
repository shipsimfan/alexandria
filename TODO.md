# ToDo
 1. Finish up basic `WaylandWindow`
   1. `xdg_surface` creations
   2. `xdg_surface_ack_configure` on `xdg_surface.configure`
   3. `xdg_toplevel` creation 
   4. Set `close_requested` on `xdg_toplevel.close`
   5. Change `size` and `is_maximized` on `xdg_toplevel.configure`
   6. Mutli-threaded user wake (eventfd)
 2. Add an optional `Input`-like callback trait for `WindowEvents`
   - `on_close_requested`
   - `on_resize(Vector2u)`
   - `on_maximize(bool)`
   - `on_focus(bool)`
   - `on_display_mode_change(DisplayMode)` (if it ever happens)
 3. Implement renderer wrappers up to clearing the screen
   1. `DebugMessenger`
   2. `Surface`
   3. `Device`
   4. `Queue`
   5. `Swapchain`
   6. `ImageView`
   7. `CommandPool`
   8. `CommandBuffer`
   9. `Semaphore`
   10. `Fence`
 4. Add Linux Windowing support
   1. Wayland
     1. Window creation
       - Display mode
     2. Thread-local window state
       - Focus
       - Display mode
     3. Window modification
       - Title
       - Size
       - Display mode
       - Request close
     4. Cursor lock to window
   2. X11
     1. Window creation
     2. `process_messages` + `wait_for_message` functions
     3. Mutli-threaded user wake (eventfd)
     4. Thread-local window state
       - Size
       - Focus
       - Display mode
       - Close requested
       - Maximized
     5. Window modification
       - Title
       - Size
       - Display mode
       - Request close
     6. Cursor lock to window
 5. Input system
   1. System
   2. Keyboard
     1. Windows
     2. Wayland
     3. X11
   3. Mouse
     1. Windows
     2. Wayland
     3. X11
   4. Cursor lock to center
     1. Windows
     2. Wayland
     3. X11
 6. Add fullscreen support
   - Windows (borderless, placed at (0, 0))
   - Wayland (extension)
   - X11
 7. Add full renderer wrappers for triangle
 8. Add model loading/parsing
 9. Add texture loading/parsing
 10. Add window icons
   - Windows
   - Wayland
   - X11
 11. Add audio wrappers
 12. Add more input types
   - X-Box controllers (Windows)
   - General controllers
   - Joysticks
   - Steering Wheels, Pedals, Gear Shift
 13. Add 2-d support to `Matrix3x3`
