pub struct Input {
    key_states: [bool; 256],
    key_down: Vec<u8>,
    key_up: Vec<u8>,
    mouse_states: [bool; 3],
    mouse_up: Vec<u8>,
    mouse_down: Vec<u8>,
    mouse_position: (isize, isize),
    mouse_lock: bool,
}

impl Input {
    pub fn new() -> Self {
        Input {
            key_states: [false; 256],
            key_down: Vec::new(),
            key_up: Vec::new(),
            mouse_states: [false; 3],
            mouse_up: Vec::new(),
            mouse_down: Vec::new(),
            mouse_position: (0, 0),
            mouse_lock: false,
        }
    }

    pub fn key_down(&mut self, key: u8) {
        if !self.key_states[key as usize] {
            self.key_down.push(key);
        }
        self.key_states[key as usize] = true;
    }

    pub fn key_up(&mut self, key: u8) {
        if self.key_states[key as usize] {
            self.key_up.push(key);
        }
        self.key_states[key as usize] = false;
    }

    pub fn mouse_down(&mut self, button: u8) {
        self.mouse_down.push(button);
        self.mouse_states[button as usize] = true;
    }

    pub fn mouse_up(&mut self, button: u8) {
        self.mouse_up.push(button);
        self.mouse_states[button as usize] = false;
    }

    pub fn set_mouse_position(&mut self, x: isize, y: isize) {
        self.mouse_position = (x, y)
    }

    pub fn set_mouse_lock(&mut self, state: bool) {
        self.mouse_lock = state;
        self.mouse_position = (0, 0)
    }

    pub fn frame_reset(&mut self) {
        self.key_down.clear();
        self.key_up.clear();
        self.mouse_up.clear();
        self.mouse_down.clear();
    }

    pub fn get_key(&self, key: u8) -> bool {
        self.key_states[key as usize]
    }

    pub fn get_key_down(&self, key: u8) -> bool {
        self.key_down.contains(&key)
    }

    pub fn get_key_up(&self, key: u8) -> bool {
        self.key_up.contains(&key)
    }

    pub fn get_mouse_button(&self, button: u8) -> bool {
        self.mouse_states[button as usize]
    }

    pub fn get_mouse_down(&self, button: u8) -> bool {
        self.mouse_down.contains(&button)
    }

    pub fn get_mouse_up(&self, button: u8) -> bool {
        self.mouse_up.contains(&button)
    }

    pub fn get_mouse_x(&self) -> isize {
        self.mouse_position.0
    }

    pub fn get_mouse_y(&self) -> isize {
        self.mouse_position.1
    }

    pub fn get_mouse_position(&self) -> (isize, isize) {
        self.mouse_position
    }

    pub fn is_mouse_locked(&self) -> bool {
        self.mouse_lock
    }
}
