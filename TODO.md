# ToDo
 1. Implement Linux callbacks for `WindowEvents`
 2. Implement renderer wrappers up to clearing the screen
   1. `Surface`
   2. `Device`
   3. `Queue`
   4. `Swapchain`
   5. `ImageView`
   6. `CommandPool`
   7. `CommandBuffer`
   8. `Semaphore`
   9. `Fence`
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
 5. Add fullscreen support
   - Windows (borderless, placed at (0, 0))
   - Wayland (extension)
   - X11
 6. Add full renderer wrappers for triangle
 7. Add model loading/parsing
 8. Add texture loading/parsing
 9. Add window icons
   - Windows
   - Wayland
   - X11
 10. Add audio wrappers
 11. Add more input types
   - X-Box controllers (Windows)
   - General controllers
   - Joysticks
   - Steering Wheels, Pedals, Gear Shift
 12. Add 2-d support to `Matrix3x3`
