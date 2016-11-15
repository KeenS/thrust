#![feature(associated_type_defaults)]

extern crate mio;
extern crate byteorder;
#[macro_use]
extern crate tokio_core;
extern crate tokio_proto;
extern crate tokio_service;
extern crate futures;
#[macro_use]
extern crate log;

pub mod protocol;
pub mod binary_protocol;
mod result;
pub mod transport;
pub mod easy;
pub mod framed_transport;

pub use result::{ThrustResult, ThrustError};
pub use protocol::{Serializer, Serialize, Deserialize, ThriftSerializer, ThriftDeserializer};
