# ToDo
 1. Add adapter enumeration
   1. Windows
      - UUID
      - Memory
   2. Linux
 2. Add platform informataion
   - CPU architecture
   - CPU model
   - CPU cores
   - Memory
   - OS name and version
   - Distro name and version (Linux Only)
 3. Add git utils (for build time)
   - Current hash
   - Current branch
 4. Add window system (Windows)
   1. Window creation
   2. `process_messages` + `wait_for_message` functions
   3. Thread-local window state
     - Window Position
     - Client Position
     - Window Size
     - Client Size
     - Focus
     - Display mode
     - Close requested
     - Minimized
     - Maximized
   2. Multi-threaded window state (feature gate `mt-window`)
   3. Window modification
   4. Mutli-threaded user wake (PostMessage, feature gate `mt-window`)
   5. Cursor lock
 5. Add Linux Windowing support
   1. Wayland
     1. Window creation
     2. `process_messages` + `wait_for_message` functions
     3. Thread-local window state
       - Window Position
       - Client Position
       - Window Size
       - Client Size
       - Focus
       - Display mode
       - Close requested
       - Minimized
       - Maximized
     2. Window modification
     3. Mutli-threaded user wake (eventfd, feature gate `mt-window`)
     4. Cursor lock
   2. X11
     1. Window creation
     2. `process_messages` + `wait_for_message` functions (user wake only with `mt-window`)
     3. Thread-local window state
       - Window Position
       - Client Position
       - Window Size
       - Client Size
       - Focus
       - Display mode
       - Close requested
       - Minimized
       - Maximized
     2. Window modification
     3. Mutli-threaded user wake (eventfd, feature gate `mt-window`)
     4. Cursor lock
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
 7. Implement renderer wrappers
   1. Device
   ...etc
 8. Add model loading/parsing
 9. Add texture loading/parsing
 10. Add audio wrappers
 11. Add more input types
 12. Add 2-d support to `Matrix3x3`