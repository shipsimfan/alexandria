# ToDo
 1. Add is_resizing tracking to coalecse resizes
   - Wayland
 2. Implement renderer wrappers up to clearing the screen
   1. `Device`
   2. `Queue`
   3. `Swapchain`
   4. `ImageView`
   5. `CommandPool`
   6. `CommandBuffer`
   7. `Semaphore`
   8. `Fence`
 3. Add Linux Windowing support
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
 4. Input system
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
 5. Add min size support
   - Windows (`WM_GETMINMAXINFO`)
   - Wayland (`xdg_toplevel_set_min_size()`)
   - X11
 6. Add fullscreen support
   - Windows (exclusive)
   - Wayland (extension)
   - X11
 7. Add renderer wrappers for triangle
 8. Add renderer wrappers for multi-cube
 9. Add model loading/parsing
 10. Add texture loading/parsing
 11. Add window icons
   - Windows
   - Wayland
   - X11
 12. Add audio wrappers
 13. Add more input types
   - X-Box controllers (Windows)
   - General controllers
   - Joysticks
   - Steering Wheels, Pedals, Gear Shift
 14. Add 2-d support to `Matrix3x3`
