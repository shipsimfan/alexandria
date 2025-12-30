use crate::GraphicsAdapterKind;

impl GraphicsAdapterKind {
    /// Get a string representing this adapter kind
    pub fn as_str(&self) -> &'static str {
        match self {
            GraphicsAdapterKind::Discrete => "discrete gpu",
            GraphicsAdapterKind::Integrated => "integrated gpu",
            GraphicsAdapterKind::Virtual => "virtual gpu",
            GraphicsAdapterKind::Cpu => "cpu",
            GraphicsAdapterKind::Other => "other",
        }
    }
}
