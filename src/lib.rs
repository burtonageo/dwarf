#[macro_use]
extern crate nom;

use std::error;
use std::fmt::{Display, Formatter, self};

pub mod parser;

#[derive(Debug)]
pub enum Error {
    ParseError
}

impl error::Error for Error {
    fn description(&self) -> &str {
        "Couldn't parse"
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "Parse error")
    }
}
