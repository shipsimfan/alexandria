use crate::input::KeyMod;

impl KeyMod {
    /// Should keys be treated as capitalized?
    pub fn caps(&self) -> bool {
        self.caps_lock ^ self.shift()
    }

    /// Is either shift key active?
    pub fn shift(&self) -> bool {
        self.left_shift || self.right_shift
    }

    /// Is either control key active?
    pub fn control(&self) -> bool {
        self.left_control || self.right_control
    }

    /// Is either alt key active?
    pub fn alt(&self) -> bool {
        self.left_alt || self.right_alt
    }

    /// Is either super key active?
    pub fn super_key(&self) -> bool {
        self.left_super || self.right_super
    }
}
