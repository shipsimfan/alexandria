# ToDo
 1. Finish math library:
   1. Complete `Matrix4x4` operations w/ tests
   2. Complete `Matrix3x3` operations w/ tests
   3. Complete `Quaternion` operations w/ tests
 2. Add adapter enumeration
   1. Windows
   2. Linux
 3. Add window system (Windows)
   1. Window creation
   2. `process_messages` function (user events only with `mt-window`)
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
   4. Mutli-threaded user events + `wait_for_message` function (mpsc + PostMessage, feature gate `mt-window`)
   5. Cursor lock
 4. Add Linux Windowing support
   1. Wayland
     1. Window creation
     2. `process_messages` function (user events only with `mt-window`)
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
     3. Mutli-threaded user events + `wait_for_message` function (mpsc + eventfd, feature gate `mt-window`)
     4. Cursor lock
   2. X11
     1. Window creation
     2. `process_messages` function (user events only with `mt-window`)
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
     3. Mutli-threaded user events + `wait_for_message` function (mpsc + eventfd, feature gate `mt-window`)
     4. Cursor lock
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