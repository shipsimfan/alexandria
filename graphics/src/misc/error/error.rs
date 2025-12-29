use crate::GraphicsError;

impl std::error::Error for GraphicsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.vk.as_ref().map(|vk| vk as _)
    }
}
