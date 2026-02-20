use crate::AlexandriaContextBuilder;

impl<UserEvent: 'static + Send> AlexandriaContextBuilder<UserEvent> {
    /// Enable the GPU subsystem
    pub fn gpu(mut self) -> Self {
        self.gpu = true;
        self
    }

    /// Enable the windowing subsystem. This implies `gpu`
    pub fn window(mut self) -> Self {
        self.window = true;
        self
    }
}
