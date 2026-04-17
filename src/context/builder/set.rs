use crate::AlexandriaContextBuilder;

impl<UserEvent: 'static + Send> AlexandriaContextBuilder<UserEvent> {
    /// Enable the GPU subsystem
    pub fn gpu(mut self) -> AlexandriaContextBuilder<UserEvent> {
        self.gpu = true;
        self
    }

    /// Enable the windowing subsystem. This implies `gpu`
    pub fn window(mut self) -> AlexandriaContextBuilder<UserEvent> {
        self.window = true;
        self
    }
}
