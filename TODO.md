# ToDo
 1. Add Linux Windowing support
   1. Wayland
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
 2. Input system
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
 3. Implement renderer wrappers
   1. Device
   ...etc
 4. Add model loading/parsing
 5. Add texture loading/parsing
 6. Add audio wrappers
 7. Add more input types
   - X-Box controllers (Windows)
   - General controllers
   - Joysticks
   - Steering Wheels, Pedals, Gear Shift
 8. Add 2-d support to `Matrix3x3`
