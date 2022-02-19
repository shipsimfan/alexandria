pub struct Input {
    key_states: [bool; 256],
    key_down: Vec<u8>,
    key_up: Vec<u8>,
}

impl Input {
    pub fn new() -> Self {
        Input {
            key_states: [false; 256],
            key_down: Vec::new(),
            key_up: Vec::new(),
        }
    }

    pub fn key_down(&mut self, key: u8) {
        self.key_down.push(key);
        self.key_states[key as usize] = true;
    }

    pub fn key_up(&mut self, key: u8) {
        self.key_up.push(key);
        self.key_states[key as usize] = false;
    }

    pub fn frame_reset(&mut self) {
        self.key_down.clear();
        self.key_up.clear();
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
}
