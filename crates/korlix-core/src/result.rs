use crate::diagnostics::DiagnosticSet;

pub type KorlixResult<T> = Result<T, KorlixError>;

#[derive(Debug)]
pub enum KorlixError {
    ParseError(DiagnosticSet),
    IoError(std::io::Error),
    ConfigError(String),
    CompileError(DiagnosticSet),
    General(String),
}

impl std::fmt::Display for KorlixError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParseError(_)   => write!(f, "Parse error"),
            Self::IoError(e)      => write!(f, "IO error: {}", e),
            Self::ConfigError(s)  => write!(f, "Config error: {}", s),
            Self::CompileError(_) => write!(f, "Compile error"),
            Self::General(s)      => write!(f, "{}", s),
        }
    }
}

impl From<std::io::Error> for KorlixError {
    fn from(e: std::io::Error) -> Self {
        Self::IoError(e)
    }
}

impl From<String> for KorlixError {
    fn from(s: String) -> Self {
        Self::General(s)
    }
}
