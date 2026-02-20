# ToDo
 1. Add display enumeration on Wayland
 2. Implement message pump on Wayland
 3. Add display related events + handling on Wayland
 4. Re-add windows on Windows
   1. Window destruction
   2. Fullscreen window creation
   3. Add minimum and maximum size handling
   4. Add Window Events
      - Close Requested
      - Moved
      - Resized
      - Minimized
      - Maximized
      - Restored
      - Focus Gained
      - Focus Lost
      - Shown
      - Hidden
      - DPI Changed
      - Enter Fullscreen
      - Leave Fullscreen
      - Destroyed
   5. Window state tracking
     - Title
     - Size
     - Minimum Size
     - Maximum Size
     - Position
     - Maximized
     - Minimized
     - Focus
     - Hidden
     - Borderless
     - Resizable
     - Is Resizing
     - Is Moving
     - Fullscreen + Fullscreen Display Mode
     - DPI
   6. Window state setting
     - Title
     - Size
     - Minimum Size
     - Maximum Size
     - Position
     - Request close
     - Maximized
     - Minimized
     - Hidden
     - Borderless
     - Resizable
     - Fullscreen + Fullscreen Display Mode
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
     5. Add Window Events
        - Close Requested
        - Moved
        - Resized
        - Minimized
        - Maximized
        - Restored
        - Focus Gained
        - Focus Lost
        - Shown
        - Hidden
        - DPI Changed
        - Enter Fullscreen
        - Leave Fullscreen
        - Destroyed
     6. Window state tracking
       - Title
       - Size
       - Minimum Size
       - Maximum Size
       - Position
       - Maximized
       - Minimized
       - Focus
       - Hidden
       - Borderless
       - Resizable
       - Is Resizing
       - Is Moving
       - Fullscreen + Fullscreen Display Mode
       - DPI
    7. Window state setting
       - Title
       - Size
       - Minimum Size
       - Maximum Size
       - Position
       - Request close
       - Maximized
       - Minimized
       - Hidden
       - Borderless
       - Resizable
       - Fullscreen + Fullscreen Display Mode
     8. Surface creation
   2. X11
     1. Connection
     2. Display enumeration
     3. `process_messages` + `wait_for_message` functions
     4. Window creation
     5. Add Window Events
        - Close Requested
        - Moved
        - Resized
        - Minimized
        - Maximized
        - Restored
        - Focus Gained
        - Focus Lost
        - Shown
        - Hidden
        - DPI Changed
        - Enter Fullscreen
        - Leave Fullscreen
        - Destroyed
     6. Window state tracking
       - Title
       - Size
       - Minimum Size
       - Maximum Size
       - Position
       - Maximized
       - Minimized
       - Focus
       - Hidden
       - Borderless
       - Resizable
       - Is Resizing
       - Is Moving
       - Fullscreen + Fullscreen Display Mode
       - DPI
    7. Window state setting
       - Title
       - Size
       - Minimum Size
       - Maximum Size
       - Position
       - Request close
       - Maximized
       - Minimized
       - Hidden
       - Borderless
       - Resizable
       - Fullscreen + Fullscreen Display Mode
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
 5. Add renderer wrappers for triangle
 6. Add renderer wrappers for multi-cube
 7. Add model loading/parsing
 8. Add texture loading/parsing
 9. Add window icons
   - Windows
   - Wayland
   - X11
 10. Add audio wrappers (pull-style)
   - WASAPI
   - PipeWire
   - PulseAudio
   - ALSA
 11. Add more input types
   - X-Box controllers (Windows)
   - General controllers
   - Joysticks
   - Steering Wheels, Pedals, Gear Shift
 12. Add 2-d support to `Matrix3x3`
 13. Add more external control to event queue
 14. Add max capacity to event queue