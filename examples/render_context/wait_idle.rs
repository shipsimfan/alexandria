use crate::render_context::RenderContext;

impl RenderContext {
    /// Wait for all activity on the GPU to stop
    pub fn wait_idle(&mut self) {
        self.device.wait_idle().unwrap();
    }
}
