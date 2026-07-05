use std::fmt;
use std::io;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    NotConsole,
    VtInUse,
    SystemdSocket,
    FontParse(&'static str),
    KeymapParse(&'static str),
    ConfigParse(&'static str),
    InvalidInput(&'static str),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(e) => write!(f, "I/O error: {}", e),
            Error::NotConsole => write!(f, "not a console device"),
            Error::VtInUse => write!(f, "no free virtual terminal available"),
            Error::SystemdSocket => write!(f, "systemd socket activation error"),
            Error::FontParse(msg) => write!(f, "font parse error: {}", msg),
            Error::KeymapParse(msg) => write!(f, "keymap error: {}", msg),
            Error::ConfigParse(msg) => write!(f, "config error: {}", msg),
            Error::InvalidInput(msg) => write!(f, "invalid input: {}", msg),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
