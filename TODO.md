# ToDo
 1. Add event system
 2. Re-add windowing
 3. Re-add surface creation
 4. Re-add device graphics items

# After re-organization
 1. Implement renderer wrappers up to clearing the screen
   1. `ImageView`
   2. `CommandPool`
   3. `CommandBuffer`
   4. `Semaphore`
   5. `Fence`
 2. Add Linux Windowing support
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
 3. Input system
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
 4. Add min size support
   - Windows (`WM_GETMINMAXINFO`)
   - Wayland (`xdg_toplevel_set_min_size()`)
   - X11
 5. Add fullscreen support
   - Windows (exclusive)
   - Wayland (extension)
   - X11
 6. Add renderer wrappers for triangle
 7. Add renderer wrappers for multi-cube
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
