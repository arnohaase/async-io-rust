use std::fmt::{Display, Formatter};

pub type P2PResult<T> = Result<T, P2PError>;

#[derive(Debug)]
pub enum P2PError {
    StdIo(std::io::Error),
    Timeout(tokio::time::error::Elapsed),
    Message(String),
}
impl Display for P2PError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl From<std::io::Error> for P2PError {
    fn from(value: std::io::Error) -> Self {
        P2PError::StdIo(value)
    }
}
impl From<tokio::time::error::Elapsed> for P2PError {
    fn from(value: tokio::time::error::Elapsed) -> Self {
        P2PError::Timeout(value)
    }
}
impl From<&str> for P2PError {
    fn from(value: &str) -> Self {
        P2PError::Message(value.into())
    }
}

#[derive(Debug, Clone)]
pub enum P2PProtocolVersion {
    //TODO arbitrary version numbers to illustrate protocol version negotiation
    V1,
    V2,
}
