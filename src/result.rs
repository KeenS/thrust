use std::io;
use std::convert;
use std::sync::PoisonError;
use byteorder;
use std::sync::mpsc::{SendError, RecvError};
use protocol;

#[derive(Debug)]
pub enum ThrustError {
    Other,
    NotReady,
    Str(String),
    IO(io::Error),
    ByteOrder(byteorder::Error),
    PoisonError,
    RecvError(RecvError),
    SendError,
}

pub type ThrustResult<T> = Result<T, ThrustError>;

impl convert::From<io::Error> for ThrustError {
    fn from(val: io::Error) -> ThrustError {
        ThrustError::IO(val)
    }
}

impl<T> convert::From<SendError<T>> for ThrustError {
    fn from(_val: SendError<T>) -> ThrustError {
        ThrustError::SendError
    }
}

impl convert::From<protocol::Error> for ThrustError {
    fn from(_val: protocol::Error) -> ThrustError {
        ThrustError::Other
    }
}

impl convert::From<RecvError> for ThrustError {
    fn from(_val: RecvError) -> ThrustError {
        ThrustError::RecvError(RecvError)
    }
}

impl convert::From<byteorder::Error> for ThrustError {
    fn from(val: byteorder::Error) -> ThrustError {
        ThrustError::ByteOrder(val)
    }
}

impl<T> convert::From<PoisonError<T>> for ThrustError {
    fn from(_val: PoisonError<T>) -> ThrustError {
        ThrustError::PoisonError
    }
}
