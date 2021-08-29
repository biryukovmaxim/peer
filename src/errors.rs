use std::error::Error;
use std::fmt::{Debug, Formatter, Display};

pub enum PeerError{
    ReadConfig(String)
}

impl Debug for PeerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            PeerError::ReadConfig(str) => write!(f, "{}", str)
        }
    }
}

impl Display for PeerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            PeerError::ReadConfig(str) => write!(f, "{}", str)
        }
    }
}

impl Error for PeerError {}