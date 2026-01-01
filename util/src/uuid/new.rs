use crate::UUID;

impl UUID {
    /// Create a new [`UUID`] from parts
    pub const fn new(
        [a0, a1, a2, a3]: [u8; 4],
        [b0, b1]: [u8; 2],
        [c0, c1]: [u8; 2],
        [d0, d1]: [u8; 2],
        [e0, e1, e2, e3, e4, e5]: [u8; 6],
    ) -> UUID {
        UUID::from_flat([
            a0, a1, a2, a3, b0, b1, c0, c1, d0, d1, e0, e1, e2, e3, e4, e5,
        ])
    }

    /// Create a new [`UUID`] from a flat array
    pub const fn from_flat(uuid: [u8; 16]) -> UUID {
        UUID { bytes: uuid }
    }
}
