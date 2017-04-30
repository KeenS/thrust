use std::io;
use std::convert;
use std::sync::PoisonError;
use std::sync::mpsc::{SendError, RecvError};
use protocol;

#[derive(Debug)]
pub enum ThriftError {
    Other,
    NotReady,
    Str(String),
    IO(io::Error),
    PoisonError,
    RecvError(RecvError),
    SendError,
}

pub type ThriftResult<T> = Result<T, ThriftError>;

impl convert::From<io::Error> for ThriftError {
    fn from(val: io::Error) -> ThriftError {
        ThriftError::IO(val)
    }
}

impl<T> convert::From<SendError<T>> for ThriftError {
    fn from(_val: SendError<T>) -> ThriftError {
        ThriftError::SendError
    }
}

impl convert::From<protocol::Error> for ThriftError {
    fn from(_val: protocol::Error) -> ThriftError {
        ThriftError::Other
    }
}

impl convert::From<RecvError> for ThriftError {
    fn from(_val: RecvError) -> ThriftError {
        ThriftError::RecvError(RecvError)
    }
}

impl<T> convert::From<PoisonError<T>> for ThriftError {
    fn from(_val: PoisonError<T>) -> ThriftError {
        ThriftError::PoisonError
    }
}
