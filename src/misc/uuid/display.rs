use crate::Uuid;

impl std::fmt::Display for Uuid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const DASHES: &[usize] = &[4, 6, 8, 10];

        for (i, byte) in self.bytes.iter().enumerate() {
            if DASHES.contains(&i) {
                write!(f, "-")?;
            }

            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}
