use crate::Adapter;

impl Adapter {
    /// Get the name of this adapter
    pub fn name(&self) -> &str {
        &self.name
    }
}
