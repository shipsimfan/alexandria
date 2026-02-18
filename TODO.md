# ToDo
 1. Add display enumeration on Wayland
 2. Implement message pump on Wayland
 3. Add display related events + handling on Wayland
 4. Re-add windows on Windows
 5. Re-add surface creation on Windows
 6. Re-add device graphics items

# After re-organization
 1. Implement renderer wrappers up to clearing the screen
   1. `ImageView`
   2. `CommandPool`
   3. `CommandBuffer`
   4. `Semaphore`
   5. `Fence`
 2. Add Linux Windowing support
   1. Wayland
     1. Connection
     2. Display enumeration
     3. `process_messages` + `wait_for_message` functions
     4. Window creation
     5. Thread-local window state
       - Size
       - Position
       - Is focused
       - Close requested
       - Is maximized
       - Is minimized
       - Is resizing
       - Is moving
       - Display styles (e.g. Is borderless)
     6. Window modification
       - Title
       - Size
       - Position
       - Request close
       - Maximized
       - Show
       - Hide
       - Display styles (e.g. borderless)
     7. Cursor lock to window
     8. Surface creation
   2. X11
     1. Connection
     2. Display enumeration
     3. `process_messages` + `wait_for_message` functions
     4. Window creation
     5. Thread-local window state
       - Size
       - Position
       - Is focused
       - Close requested
       - Is maximized
       - Is minimized
       - Is resizing
       - Is moving
       - Display styles (e.g. Is borderless)
     6. Window modification
       - Title
       - Size
       - Position
       - Request close
       - Maximized
       - Show
       - Hide
       - Display styles (e.g. borderless)
     7. Cursor lock to window
     8. Surface creation
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
 14. Add more external control to event queue
 15. Add max capacity to event queue