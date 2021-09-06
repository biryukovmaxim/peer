use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::net::AddrParseError;

pub enum PeerError {
    ReadConfig(String),
    ParseServerAddressError(String),
    CannotStartServer(String),
    Other(String),
}

impl From<tonic::transport::Error> for PeerError {
    fn from(err: tonic::transport::Error) -> Self {
        PeerError::Other(err.to_string())
    }
}

impl From<AddrParseError> for PeerError {
    fn from(err: AddrParseError) -> Self {
        PeerError::ParseServerAddressError(err.to_string())
    }
}

impl Debug for PeerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            PeerError::ReadConfig(str) => write!(f, "config reading error: {}", str),
            PeerError::ParseServerAddressError(str) => {
                write!(f, "server address parsing error: {}", str)
            }
            PeerError::Other(err) => write!(f, "unknown error: {}", err),
            PeerError::CannotStartServer(err) => write!(f, "can't start server error: {}", err)
        }
    }
}

impl Display for PeerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            PeerError::ReadConfig(str) => write!(f, "config reading error: {}", str),
            PeerError::ParseServerAddressError(str) => {
                write!(f, "server address parsing error: {}", str)
            }
            PeerError::Other(err) => write!(f, "unknown error: {}", err),
            PeerError::CannotStartServer(err) => write!(f, "can't start server error: {}", err)
        }
    }
}

impl Error for PeerError {}
