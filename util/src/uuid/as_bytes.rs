use crate::UUID;

impl UUID {
    /// Get the bytes of this [`UUID`]
    pub const fn as_bytes(&self) -> &[u8; 16] {
        &self.bytes
    }

    /// Get the bytes of this [`UUID`] as a slice
    pub const fn as_slice(&self) -> &[u8] {
        self.bytes.as_slice()
    }
}
