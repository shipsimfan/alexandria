use crate::input::{KeyCode, KeyMod};

impl KeyMod {
    /// Apply a key down event to this [`KeyMod`]
    pub(crate) fn apply_key_down(&mut self, key_code: KeyCode) {
        match key_code {
            KeyCode::LeftShift => self.left_shift = true,
            KeyCode::RightShift => self.right_shift = true,
            KeyCode::LeftControl => self.left_control = true,
            KeyCode::RightControl => self.right_control = true,
            KeyCode::LeftAlt => self.left_alt = true,
            KeyCode::RightAlt => self.right_alt = true,
            KeyCode::LeftSuper => self.left_super = true,
            KeyCode::RightSuper => self.right_super = true,
            KeyCode::CapsLock => self.caps_lock = !self.caps_lock,
            KeyCode::NumLock => self.num_lock = !self.num_lock,
            KeyCode::ScrollLock => self.scroll_lock = !self.scroll_lock,
            _ => {}
        }
    }

    /// Apply a key up event to this [`KeyMod`]
    pub(crate) fn apply_key_up(&mut self, key_code: KeyCode) {
        match key_code {
            KeyCode::LeftShift => self.left_shift = false,
            KeyCode::RightShift => self.right_shift = false,
            KeyCode::LeftControl => self.left_control = false,
            KeyCode::RightControl => self.right_control = false,
            KeyCode::LeftAlt => self.left_alt = false,
            KeyCode::RightAlt => self.right_alt = false,
            KeyCode::LeftSuper => self.left_super = false,
            KeyCode::RightSuper => self.right_super = false,
            _ => {}
        }
    }
}
