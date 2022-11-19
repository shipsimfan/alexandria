use crate::Input;
use alexandria_common::{Key, MouseButton};

pub struct StateTrackingInput {
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

impl Input for StateTrackingInput {
    fn new() -> Self {
        StateTrackingInput {
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

    fn key_down(&mut self, key: Key) {
        if !self.key_states[key as usize] {
            self.key_states_down[key as usize] = true;
        }
        self.key_states[key as usize] = true;
    }

    fn key_up(&mut self, key: Key) {
        if self.key_states[key as usize] {
            self.key_states_up[key as usize] = true;
        }
        self.key_states[key as usize] = false;
    }

    fn mouse_down(&mut self, button: MouseButton) {
        if !self.mouse_states[button as usize] {
            self.mouse_states_down[button as usize] = true;
        }
        self.mouse_states[button as usize] = true;
    }

    fn mouse_up(&mut self, button: MouseButton) {
        if self.mouse_states[button as usize] {
            self.mouse_states_up[button as usize] = true;
        }
        self.mouse_states[button as usize] = false;
    }

    fn update_mouse_position(&mut self, position: (isize, isize)) {
        self.mouse_position = position;
    }

    fn set_mouse_lock(&mut self, state: bool) {
        self.mouse_lock = state;

        self.mouse_position = (0, 0);
    }

    fn is_mouse_locked(&self) -> bool {
        self.mouse_lock
    }

    fn frame_reset(&mut self) {
        for i in 0..NUM_KEYS {
            self.key_states_down[i] = false;
            self.key_states_up[i] = false;
        }

        for i in 0..NUM_BUTTONS {
            self.mouse_states_down[i] = false;
            self.mouse_states_up[i] = false;
        }
    }
}

impl StateTrackingInput {
    pub fn get_key(&self, key: Key) -> bool {
        self.key_states[key as usize]
    }

    pub fn get_key_down(&self, key: Key) -> bool {
        self.key_states_down[key as usize]
    }

    pub fn get_key_up(&self, key: Key) -> bool {
        self.key_states_up[key as usize]
    }

    pub fn get_mouse_button(&self, button: MouseButton) -> bool {
        self.mouse_states[button as usize]
    }

    pub fn get_mouse_down(&self, button: MouseButton) -> bool {
        self.mouse_states_down[button as usize]
    }

    pub fn get_mouse_up(&self, button: MouseButton) -> bool {
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
}
