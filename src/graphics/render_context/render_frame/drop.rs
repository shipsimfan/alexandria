use crate::graphics::RenderFrame;

impl<'a> Drop for RenderFrame<'a> {
    fn drop(&mut self) {
        if !self.frame_ended {
            panic!("Must call `end` on `RenderFrame`");
        }
    }
}
