# ToDo
 1. Add platform informataion
   1. Linux
     - CPU cores
     - Memory
     - OS name and version
     - Hostname
     - Distro name and version (Linux Only)
 2. Add git utils (for build time)
   - Current hash
   - Current branch
 3. Add window system (Windows)
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
 4. Add Linux Windowing support
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
 6. Implement renderer wrappers
   1. Device
   ...etc
 7. Add model loading/parsing
 8. Add texture loading/parsing
 9. Add audio wrappers
 10. Add more input types
   - X-Box controllers (Windows)
   - General controllers
   - Joysticks
   - Steering Wheels, Pedals, Gear Shift
 11. Add 2-d support to `Matrix3x3`
