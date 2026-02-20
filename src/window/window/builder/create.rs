use crate::{
    Result,
    window::{Window, WindowBuilder},
};

impl<UserEvent: 'static + Send> WindowBuilder<UserEvent> {
    /// Create a new [`Window`] with the settings of this builder
    pub fn create(&self) -> Result<Window<UserEvent>> {
        self.context.do_create_window(self)
    }
}
