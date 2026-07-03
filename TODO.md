# ToDo
 1. Input system
   1. Keyboard
     1. Wayland
     2. X11
   2. Mouse
     1. Windows
     2. Wayland
     3. X11
   3. Cursor lock to center
     1. Windows
     2. Wayland
     3. X11
 2. Add error handling for pushed events during event pumping
   - Linux
 3. Add renderer wrappers for triangle
 4. Add renderer wrappers for multi-cube
 5. Add model loading/parsing
   1. obj
   2. fbx
   3. gltf/glb
   4. dae
 6. Add texture loading/parsing
   1. qoi
   2. png
   3. jpg
   4. tga
   5. bmp
   6. dds
   7. ktx2
   8. exr
 7. Finish adding Linux Windowing support
   1. Wayland
     1. Add extra window Events
       - Content Scale Changed
     2. Add extra state tracking
       - Content Scale
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
 8. Add window icons
   1. Windows
   2. Wayland
   3. X11
 9. Add audio wrappers (pull-style)
   1. WASAPI
   2. PipeWire
   3. PulseAudio
   4. ALSA
 10. Add audio file loading/parsing
   1. wav
   2. ogg
   3. mp3
   4. flac
   5. opus
 11. Add support for multiple keyboards
   1. Windows
   2. Wayland
   3. X11
 12. Add support for multiple mice
   1. Windows
   2. Wayland
   3. X11
 13. Add more input types
   1. X-Box controllers (Windows)
   2. General controllers
   3. Joysticks
   4. Steering Wheels, Pedals, Gear Shift
 14. Add 2-d support to `Matrix3x3`
 15. Add more external control to event queue
 16. Add max capacity to event queue
 17. Make fullscreen look for display with greatest overlap