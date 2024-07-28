use super::functions::GlobalFunctions;

mod instance_extensions;
mod instance_layers;
mod new;

/// Global Vulkan functions
pub struct Global {
    functions: GlobalFunctions,
}

impl Global {
    /// Gets the global level functions
    pub(crate) fn f(&self) -> &GlobalFunctions {
        &self.functions
    }
}
