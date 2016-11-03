#![feature(associated_type_defaults, question_mark)]

extern crate mio;
extern crate byteorder;

pub mod protocol;
pub mod binary_protocol;
mod result;
pub mod transport;

pub use result::{ThrustResult, ThrustError};
pub use protocol::{Serializer, Serialize, Deserialize, ThriftSerializer, ThriftDeserializer};
