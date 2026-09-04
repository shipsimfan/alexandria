# ToDo
 1. Add error handling for keyboard events during event pumping
   - Wayland
 2. Add model loading/parsing (Parthenon)
   1. obj
   2. fbx
   3. gltf/glb
   4. dae
 3. Add texture loading/parsing (Mosaic)
   1. qoi
   2. png
   3. jpg
   4. tga
   5. bmp
   6. dds
   7. ktx2
   8. exr
 4. Input system
   1. Add keyboard window tracking to Wayland
   2. Mouse
     1. Windows
     2. Wayland
   3. Cursor lock to center
     1. Windows
     2. Wayland
   4. X-Box controllers (Windows)
 5. Add window icons
   1. Windows
   2. Wayland
 6. Add audio wrappers (pull-style)
   1. WASAPI
   2. PipeWire
   3. PulseAudio
   4. ALSA
 7. Add audio file loading/parsing
   1. wav
   2. ogg
   3. mp3
   4. flac
   5. opus
 8. Add text input support
   1. Windows
   2. Wayland
 9. Add support for multiple keyboards
   1. Windows
   2. Wayland
 10. Add support for multiple mice
   1. Windows
   2. Wayland
 11. Add more input types
   1. General controllers
   2. Joysticks
   3. Steering Wheels, Pedals, Gear Shift
 12. Add 2-d support to `Matrix3x3`
 13. Add more external control to event queue
 14. Add max capacity to event queue
 15. Make fullscreen look for display with greatest overlap
 16. Add dynamic `libdecor` support for Wayland
 17. Finish Wayland window support
   1. Add extra window Events
     - Content Scale Changed
   2. Add extra state tracking
     - Content Scale
 18. Add X11 support
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
   10. Keyboard input
   11. Mouse input
   12. Cursor lock to center
   13. Window icons
   14. Text input support