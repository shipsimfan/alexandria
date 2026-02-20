use crate::window::WindowSubsystem;

impl<UserEvent: 'static + Send> Clone for WindowSubsystem<UserEvent> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}
