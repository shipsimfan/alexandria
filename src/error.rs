use crate::os;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Copy)]
pub enum ErrorKind {
    VulkanInstanceCreationFailed,
    OsInstanceCreationFailed,
    WindowCreationFailed,
    PollEventsFailed,
}

pub enum ErrorSource {
    OS(os::Error),
    Vulkan(vulkan::VkResult),
}

pub struct Error {
    kind: ErrorKind,
    source: Option<ErrorSource>,
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ErrorKind::*;

        write!(
            f,
            "{}",
            match self {
                VulkanInstanceCreationFailed => "Vulkan instance creation failed",
                OsInstanceCreationFailed => "OS instance creation failed",
                WindowCreationFailed => "Window creation failed",
                PollEventsFailed => "Polling window events failed",
            }
        )
    }
}

impl std::fmt::Debug for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl std::error::Error for ErrorSource {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(match self {
            ErrorSource::OS(os) => os,
            ErrorSource::Vulkan(vulkan) => vulkan,
        })
    }
}

impl std::fmt::Display for ErrorSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorSource::OS(os) => os.fmt(f),
            ErrorSource::Vulkan(vulkan) => vulkan.fmt(f),
        }
    }
}

impl std::fmt::Debug for ErrorSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

macro_rules! create_error {
    ($kind: ident, $source: expr) => {{
        use $crate::ErrorSource::*;
        $crate::Error::new($crate::ErrorKind::$kind, $source)
    }};
}
pub(crate) use create_error;

impl Error {
    pub(crate) fn new(kind: ErrorKind, source: Option<ErrorSource>) -> Self {
        Error { kind, source }
    }

    pub fn kind(&self) -> ErrorKind {
        self.kind
    }

    pub fn source(&self) -> Option<&ErrorSource> {
        self.source.as_ref()
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|source| source.source().unwrap())
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind)?;

        match &self.source {
            Some(source) => write!(f, " ({})", source),
            None => Ok(()),
        }
    }
}
