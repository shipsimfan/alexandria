use crate::RenderContext;

impl<'a> Drop for RenderContext<'a> {
    fn drop(&mut self) {
        if !self.frame_ended {
            panic!("Must call `end` on `RenderContext`");
        }
    }
}
