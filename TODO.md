# ToDo
 1. Add window system (Windows)
   1. Window creation
   2. `process_messages` + `wait_for_message` functions
   3. Mutli-threaded user wake (PostMessage)
   4. Thread-local window state
     - Window Position
     - Client Position
     - Window Size
     - Client Size
     - Focus
     - Display mode
     - Close requested
     - Minimized
     - Maximized
   5. Window modification
   6. Cursor lock
 2. Add Linux Windowing support
   1. Wayland
     1. Window creation
     2. `process_messages` + `wait_for_message` functions
     3. Mutli-threaded user wake (eventfd)
     4. Thread-local window state
       - Window Position
       - Client Position
       - Window Size
       - Client Size
       - Focus
       - Display mode
       - Close requested
       - Minimized
       - Maximized
     5. Window modification
     6. Cursor lock
   2. X11
     1. Window creation
     2. `process_messages` + `wait_for_message` functions
     3. Mutli-threaded user wake (eventfd)
     4. Thread-local window state
       - Window Position
       - Client Position
       - Window Size
       - Client Size
       - Focus
       - Display mode
       - Close requested
       - Minimized
       - Maximized
     5. Window modification
     6. Cursor lock
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
 4. Implement renderer wrappers
   1. Device
   ...etc
 5. Add model loading/parsing
 6. Add texture loading/parsing
 7. Add audio wrappers
 8. Add more input types
   - X-Box controllers (Windows)
   - General controllers
   - Joysticks
   - Steering Wheels, Pedals, Gear Shift
 9. Add 2-d support to `Matrix3x3`
