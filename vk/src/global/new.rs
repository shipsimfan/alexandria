use super::Global;
use crate::{functions::GlobalFunctions, CreateError};

impl Global {
    /// Loads the global Vulkan functions
    pub fn new() -> Result<Self, CreateError> {
        let functions = GlobalFunctions::new()?;

        Ok(Global { functions })
    }
}
