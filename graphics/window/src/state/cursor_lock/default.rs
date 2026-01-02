use crate::CursorLock;

impl Default for CursorLock {
    fn default() -> Self {
        CursorLock::Unlocked
    }
}
