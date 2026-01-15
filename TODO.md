# ToDo
 1. Add Wayland surface creation
 2. Add is_resizing tracking to coalecse resizes
   - Windows
   - Wayland
 3. Update Windows `DisplayMode::Borderless` to set position to `(0, 0)`
 4. Implement renderer wrappers up to clearing the screen
   1. `Surface` for Wayland
   2. `Device`
   3. `Queue`
   4. `Swapchain`
   5. `ImageView`
   6. `CommandPool`
   7. `CommandBuffer`
   8. `Semaphore`
   9. `Fence`
 5. Add Linux Windowing support
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
     7. Surface creation
 6. Input system
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
 7. Add min size support
   - Windows (`WM_GETMINMAXINFO`)
   - Wayland (`xdg_toplevel_set_min_size()`)
   - X11
 8. Add fullscreen support
   - Windows (exclusive)
   - Wayland (extension)
   - X11
 9. Add renderer wrappers for triangle
 10. Add renderer wrappers for multi-cube
 11. Add model loading/parsing
 12. Add texture loading/parsing
 13. Add window icons
   - Windows
   - Wayland
   - X11
 14. Add audio wrappers
 15. Add more input types
   - X-Box controllers (Windows)
   - General controllers
   - Joysticks
   - Steering Wheels, Pedals, Gear Shift
 16. Add 2-d support to `Matrix3x3`
