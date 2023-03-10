#[derive(Clone, Copy)]
pub enum DebugMessageLevel {
    Fatal,
    Error,
    Warning,
    Info,
}

pub struct DebugMessage {
    message: String,
    level: DebugMessageLevel,
}

impl DebugMessage {
    pub fn new(message: String, level: DebugMessageLevel) -> Self {
        DebugMessage { message, level }
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn level(&self) -> DebugMessageLevel {
        self.level
    }
}

impl std::fmt::Display for DebugMessageLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DebugMessageLevel::Fatal => "Fatal",
                DebugMessageLevel::Error => "Error",
                DebugMessageLevel::Warning => "Warning",
                DebugMessageLevel::Info => "Info",
            }
        )
    }
}
