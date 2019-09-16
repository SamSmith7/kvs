use bincode;
use std::error;
use std::fmt;
use std::io;


/// Error type for Kv Store
#[derive(Debug)]
pub enum KvError {
    /// File IO error type
    Io(io::Error),
    /// Serialisation error
    Serialize(bincode::Error),
}

impl fmt::Display for KvError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match * self {
            KvError::Io(ref err) => write!(f, "IO Error: {}", err),
            KvError::Serialize(ref err) => write!(f, "Serialization Error: {}", err),
        }
    }
}

impl error::Error for KvError {
    fn description(&self) -> &str {
        match *self {
            KvError::Io(ref err) => err.description(),
            KvError::Serialize(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            KvError::Io(ref err) => Some(err),
            KvError::Serialize(ref err) => Some(err)
        }
    }
}

impl From<io::Error> for KvError {
    fn from(err: io::Error) -> KvError {
        KvError::Io(err)
    }
}

impl From<bincode::Error> for KvError {
    fn from(err: bincode::Error) -> KvError {
        KvError::Serialize(err)
    }
}

/// A result type for KvStore
pub type Result<T> = std::result::Result<T, KvError>;
