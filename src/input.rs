pub struct Input {
    key_states: [bool; NUM_KEYS],
    key_states_down: [bool; NUM_KEYS],
    key_states_up: [bool; NUM_KEYS],
    mouse_states: [bool; NUM_BUTTONS],
    mouse_states_down: [bool; NUM_BUTTONS],
    mouse_states_up: [bool; NUM_BUTTONS],
    mouse_position: (isize, isize),
    mouse_lock: bool,
}

const NUM_KEYS: usize = 256;
const NUM_BUTTONS: usize = 3;

impl Input {
    pub fn new() -> Self {
        Input {
            key_states: [false; NUM_KEYS],
            key_states_down: [false; NUM_KEYS],
            key_states_up: [false; NUM_KEYS],
            mouse_states: [false; NUM_BUTTONS],
            mouse_states_down: [false; NUM_BUTTONS],
            mouse_states_up: [false; NUM_BUTTONS],
            mouse_position: (0, 0),
            mouse_lock: false,
        }
    }

    pub fn key_down(&mut self, key: u8) {
        self.key_states_down[key as usize] = true;
        self.key_states[key as usize] = true;
    }

    pub fn key_up(&mut self, key: u8) {
        self.key_states_up[key as usize] = true;
        self.key_states[key as usize] = false;
    }

    pub fn mouse_down(&mut self, button: u8) {
        self.mouse_states_down[button as usize] = true;
        self.mouse_states[button as usize] = true;
    }

    pub fn mouse_up(&mut self, button: u8) {
        self.mouse_states_up[button as usize] = true;
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
        for i in 0..NUM_KEYS {
            self.key_states_down[i] = false;
            self.key_states_up[i] = false;
        }

        for i in 0..NUM_BUTTONS {
            self.mouse_states_down[i] = false;
            self.mouse_states_up[i] = false;
        }
    }

    pub fn get_key(&self, key: u8) -> bool {
        self.key_states[key as usize]
    }

    pub fn get_key_down(&self, key: u8) -> bool {
        self.key_states_down[key as usize]
    }

    pub fn get_key_up(&self, key: u8) -> bool {
        self.key_states_up[key as usize]
    }

    pub fn get_mouse_button(&self, button: u8) -> bool {
        self.mouse_states[button as usize]
    }

    pub fn get_mouse_down(&self, button: u8) -> bool {
        self.mouse_states_down[button as usize]
    }

    pub fn get_mouse_up(&self, button: u8) -> bool {
        self.mouse_states_up[button as usize]
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
