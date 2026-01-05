use crate::{
    Result,
    platform::linux::wayland::{WaylandGlobals, WlCompositor},
};

impl WaylandGlobals {
    /// Get the result from the last dispatch
    ///
    /// This function takes the result, setting it back to `Ok(())`
    pub fn result(&mut self) -> Result<()> {
        let mut result = Ok(());
        std::mem::swap(&mut result, &mut self.dispatch_result);
        result
    }

    /// Get the global compositor
    pub fn compositor(&self) -> Option<&WlCompositor> {
        self.compositor.as_ref()
    }
}
