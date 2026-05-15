# ToDo
 1. Add frames in flight support to blank-window
 2. Generalize rendering from blank-window to be used in other examples
 3. Improve current Vulkan wrappers to be closer to spec
 4. Fix borderless window issue on Windows
 5. Add error handling for pushed events during event pumping
   - Linux
   - Windows
 6. Add Linux Windowing support
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
       - Content Scale Changed
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
       - Content Scale
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
 7. Add functions for getting system paths
   - Home
     - Linux: `$HOME`, then `getpwuid(getuid())->pw_dir`
   - Desktop 
     - Linux: `$XDG_DESKTOP_DIR`, then `home()/Desktop`
   - Game Data (take app name and company name as arguments)
     - Linux: `$XDG_DATA_HOME/<app-name>`, then `home()/.local/share/<app-name>`
   - Cache (take app name and company name as arguments)
     - Linux: `$XDG_CACHE_HOME/<app-name>`, then `home()/.cache/<app-name>`
   - Temp
     - Linux: `$TMPDIR`, then `/tmp/`
   - Downloads
     - Linux: `$XDG_DOWNLOAD_DIR`, then `home()/Downloads/`
   - Documents
     - Linux: `$XDG_DOCUMENTS_DIR`, then `home()/Documents/`
   - Pictures
     - Linux: `$XDG_PICTURES_DIR`, then `home()/Pictures/`
   - Screenshots
     - Linux: `$XDG_PICTURES_DIR/Screenshots/`, then `home()/Pictures/Screenshots/`
   - Music
     - Linux: `$XDG_MUSIC_DIR`, then `home()/Music/`
   - Videos
      - Linux: `$XDG_VIDEOS_DIR`, then `home()/Videos/`
 8. Input system
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
 9. Add min size support
   - Windows (`WM_GETMINMAXINFO`)
   - Wayland (`xdg_toplevel_set_min_size()`)
   - X11
 10. Add renderer wrappers for triangle
 11. Add renderer wrappers for multi-cube
 12. Add model loading/parsing
   1. obj
   2. fbx
   3. gltf/glb
   4. dae
 13. Add texture loading/parsing
   1. qoi
   2. png
   3. jpg
   4. tga
   5. bmp
   6. dds
   7. ktx2
   8. exr
 14. Add window icons
   1. Windows
   2. Wayland
   3. X11
 15. Add audio wrappers (pull-style)
   1. WASAPI
   2. PipeWire
   3. PulseAudio
   4. ALSA
 16. Add audio file loading/parsing
   1. wav
   2. ogg
   3. mp3
   4. flac
   5. opus
 17. Add more input types
   1. X-Box controllers (Windows)
   2. General controllers
   3. Joysticks
   4. Steering Wheels, Pedals, Gear Shift
 18. Add 2-d support to `Matrix3x3`
 19. Add more external control to event queue
 20. Add max capacity to event queue