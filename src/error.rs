use std::convert::From;
use std::error;
use std::io;
use std::fmt::{Display, Formatter, self};

#[derive(Debug)]
pub enum Error {
    ParseError,
    IoError(io::Error)
}

impl From<io::Error> for Error {
    fn from(io_err: io::Error) -> Self {
        Error::IoError(io_err)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        "Couldn't parse"
    }

    fn cause(&self) -> Option<&error::Error> {
        match self {
            &Error::ParseError => None,
            &Error::IoError(ref e) => Some(e)
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "Parse error")
    }
}
