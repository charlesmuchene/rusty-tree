use std::{
    fmt::{self, Formatter},
    io,
};

use RustyError::IOError;

/// Custom error for the program
pub enum RustyError {
    /// Wrapper around an `io::Error`
    IOError(io::Error),
}

impl fmt::Display for RustyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            // FIXME What would be a suitable display of this error?
            IOError(e) => f.write_fmt(format_args!("{e}")),
        }
    }
}

impl From<io::Error> for RustyError {
    fn from(value: io::Error) -> Self {
        IOError(value)
    }
}