# ToDo
 1. Add error handling for pushed events during event pumping
   - Linux
   - Windows
 2. Re-add windows on Windows
   1. Add Window Events
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
   2. Window state tracking
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
   3. Window state setting
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
 3. Re-add surface creation on Windows
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
     2. Window destruction
     3. Surface creation
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
   2. X11
     1. Connection
     2. `pump_events` + `wait_for_event` functions
     3. Display enumeration
     4. Window creation
     5. Window destruction
     6. Surface creation
     7. Add Window Events
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
     8. Window state tracking
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
     9. Window state setting
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
   1. obj
   2. fbx
   3. gltf/glb
   4. dae
 8. Add texture loading/parsing
   1. qoi
   2. png
   3. jpg
   4. tga
   5. bmp
   6. dds
   7. ktx2
   8. exr
 9. Add window icons
   1. Windows
   2. Wayland
   3. X11
 10. Add audio wrappers (pull-style)
   1. WASAPI
   2. PipeWire
   3. PulseAudio
   4. ALSA
 11. Add audio file loading/parsing
   1. wav
   2. ogg
   3. mp3
   4. flac
   5. opus
 12. Add more input types
   1. X-Box controllers (Windows)
   2. General controllers
   3. Joysticks
   4. Steering Wheels, Pedals, Gear Shift
 13. Add 2-d support to `Matrix3x3`
 14. Add more external control to event queue
 15. Add max capacity to event queue