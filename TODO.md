# ToDo
 1. Improve current Vulkan wrappers to be closer to spec
   - Command pools (include flag)
 2. Add Linux Windowing support
   1. Wayland
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
       - Content Scale Changed
       - Enter Fullscreen
       - Leave Fullscreen
     2. Window state tracking
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
       - Content Scale
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
       - Content Scale Changed
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
       - Content Scale
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
 3. Add `ColorHsv` and `ColorHsva`
 4. Add error handling for pushed events during event pumping
   - Linux
   - Windows
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
   4. Cursor lock to center
     1. Windows
     2. Wayland
     3. X11
 6. Add renderer wrappers for triangle
 7. Add renderer wrappers for multi-cube
 8. Add model loading/parsing
   1. obj
   2. fbx
   3. gltf/glb
   4. dae
 9. Add texture loading/parsing
   1. qoi
   2. png
   3. jpg
   4. tga
   5. bmp
   6. dds
   7. ktx2
   8. exr
 10. Add window icons
   1. Windows
   2. Wayland
   3. X11
 11. Add audio wrappers (pull-style)
   1. WASAPI
   2. PipeWire
   3. PulseAudio
   4. ALSA
 12. Add audio file loading/parsing
   1. wav
   2. ogg
   3. mp3
   4. flac
   5. opus
 13. Add more input types
   1. X-Box controllers (Windows)
   2. General controllers
   3. Joysticks
   4. Steering Wheels, Pedals, Gear Shift
 14. Add 2-d support to `Matrix3x3`
 15. Add more external control to event queue
 16. Add max capacity to event queue