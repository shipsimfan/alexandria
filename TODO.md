# ToDo
 1. Implement Linux callbacks for `WindowEvents`
   1. On resize
   2. On close requested
   3. On maximized
   4. On restore
 2. Update Windows `DisplayMode::Borderless` to set position to `(0, 0)`
 3. Implement renderer wrappers up to clearing the screen
   1. `Surface` for Wayland
   2. `Device`
   3. `Queue`
   4. `Swapchain`
   5. `ImageView`
   6. `CommandPool`
   7. `CommandBuffer`
   8. `Semaphore`
   9. `Fence`
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
     7. Surface creation
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
 6. Add min size support
   - Windows (`WM_GETMINMAXINFO`)
   - Wayland (`xdg_toplevel_set_min_size()`)
   - X11
 7. Add fullscreen support
   - Windows (exclusive)
   - Wayland (extension)
   - X11
 8. Add renderer wrappers for triangle
 9. Add renderer wrappers for multi-cube
 10. Add model loading/parsing
 11. Add texture loading/parsing
 12. Add window icons
   - Windows
   - Wayland
   - X11
 13. Add audio wrappers
 14. Add more input types
   - X-Box controllers (Windows)
   - General controllers
   - Joysticks
   - Steering Wheels, Pedals, Gear Shift
 15. Add 2-d support to `Matrix3x3`
