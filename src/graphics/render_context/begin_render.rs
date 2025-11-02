use win32::{d3d11::D3D11_PRIMITIVE_TOPOLOGY, UINT};

use crate::{
    graphics::{RenderContext, RenderFrame},
    LogCallbacks, Result,
};

impl RenderContext {
    /// Begin rendering a new frame
    pub(crate) fn begin_render<'a>(
        &'a mut self,
        log_callbacks: &'a mut dyn LogCallbacks,
        vsync: bool,
        clear_color: [f32; 4],
    ) -> Result<RenderFrame<'a>> {
        // Clear debug message queue from object creation
        #[cfg(debug_assertions)]
        self.info_queue.empty_queue(log_callbacks)?;

        // Clear set-state variables
        self.current_shader = None;

        // Set global render state variables
        self.device_context
            .ia_set_primitive_topology(D3D11_PRIMITIVE_TOPOLOGY::TriangleList);
        self.device_context
            .rs_set_state(self.rasterizer_state.as_mut());
        self.device_context
            .om_set_blend_state(self.blend_state.as_mut(), [1.; 4], UINT::MAX);
        self.device_context
            .om_set_depth_stencil_state(self.depth_stencil_state.as_mut(), 0);

        self.swapchain_objects
            .as_mut()
            .unwrap()
            .bind(&mut self.device_context);

        // Return frame reference object
        Ok(RenderFrame::new(self, log_callbacks, vsync, clear_color))
    }
}
