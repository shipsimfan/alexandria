use crate::{Result, window::subsystem::linux::wayland::WaylandGlobals};

impl WaylandGlobals {
    /// Get the result from the last dispatch
    ///
    /// This function takes the result, setting it back to `Ok(())`
    pub fn result(&mut self) -> Result<()> {
        let mut result = Ok(());
        std::mem::swap(&mut result, &mut self.dispatch_result);
        result
    }

    /*
    /// Get a reference to the global compositor
    pub fn compositor(&self) -> Option<&WlCompositor> {
        self.compositor.as_ref()
    }

    /// Get a reference to the XDG window manager
    pub fn xdg_wm_base(&self) -> Option<&Rc<XdgWmBase>> {
        self.xdg_wm_base.as_ref()
    }

    /// Get a mutable reference to the global compositor
    pub fn compositor_mut(&mut self) -> Option<&mut WlCompositor> {
        self.compositor.as_mut()
    }
    */
}
