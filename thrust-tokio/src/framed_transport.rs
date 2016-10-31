use std::io::{self, Cursor};
use tokio_core::io::Io;
use tokio_core::easy::{Parse, Serialize, EasyBuf, EasyFramed};
use thrust::protocol::{Deserialize as De, Serialize as Se, Deserializer, Serializer,ThriftDeserializer, ThriftSerializer, Error, ThriftMessageType};
use thrust::binary_protocol::BinaryProtocol;
use thrust::transport::*;
use futures::{Poll, Async};
use std::marker::PhantomData;

pub struct TTransport<T> {
    phantom: PhantomData<T>
}

impl <T>TTransport<T> {
    pub fn new() -> Self {
        TTransport {
            phantom: PhantomData
        }
    }
}

macro_rules! try_async {
    ($t:expr) => (
        match $t {
            Ok(res) => res,
            Err(Error::Byteorder(_)) => return Ok(Async::NotReady),
            Err(Error::Io(e)) => {println!("ioerro");return Err(e)},
            Err(e) => return {println!("othreerro {:?}", e);Err(io::Error::new(io::ErrorKind::InvalidData, "failed to parse thrift data"))},
        }
    )
}
impl <T>Parse for TTransport<T>
    where T: De,
{
    type Out = T;

    fn parse(&mut self, buf: &mut EasyBuf) -> Poll<Self::Out, io::Error> {
        let cur = Cursor::new(buf);
        let mut protocol = BinaryProtocol::from(cur);
        let ret = try_async!(De::deserialize(&mut protocol));
        let cur = protocol.into_inner();
        let size = cur.position();
        let buf = cur.into_inner();
        buf.drain_to((size+1) as usize);
        Ok(Async::Ready(ret))
    }
}

impl <T>Serialize for TTransport<T>
    where T: Se,
{
    type In = T;

    fn serialize(&mut self, frame: Self::In, buf: &mut Vec<u8>) {
        let mut protocol = BinaryProtocol::from(buf);
        let _ = frame.serialize(&mut protocol);
    }
}


pub trait ParseThrift {
    type Args;
    type Ret;
    fn parse_args<D: Deserializer + ThriftDeserializer>(&self, buf: &mut D) -> Result<Self::Args, Error>;
    fn parse_ret<D: Deserializer + ThriftDeserializer>(&self, buf: &mut D) -> Result<Self::Ret, Error>;
}



pub struct TMethodArgsTransport<T> {
    phantom: PhantomData<T>
}

impl <T>TMethodArgsTransport<T> {
    pub fn new() -> Self {
        TMethodArgsTransport {
            phantom: PhantomData
        }
    }
}

pub struct TMethodRetTransport<T> {
    phantom: PhantomData<T>
}

impl <T>TMethodRetTransport<T> {
    pub fn new() -> Self {
        TMethodRetTransport {
            phantom: PhantomData
        }
    }
}

use std::str::FromStr;

macro_rules! gen_parse {
    ($stct:ident, $out:ident, $parser:ident, $msg_type: ident) => {
        impl <T>Parse for $stct<T>
            where T: ParseThrift + FromStr + Sized,
        {
            type Out = T::$out;

            fn parse(&mut self, buf: &mut EasyBuf) -> Poll<Self::Out, io::Error> {
                println!("called");
                let cur = Cursor::new(buf);
                let mut protocol = BinaryProtocol::from(cur);
                let msg = try_async!(protocol.read_message_begin());
                println!("called2");
                //assert!(msg.type) == $msg_type
                println!("got {}", msg.name);
                let ret = match T::from_str(&msg.name) {
                    Ok(method) => try_async!(method.$parser(&mut protocol)),
                    Err(_) => return Err(io::Error::new(io::ErrorKind::InvalidData, "failed to parse thrift data")),
                };
                let _ = try_async!(protocol.read_message_end());
                let cur = protocol.into_inner();
                let size = cur.position();
                let buf = cur.into_inner();
                buf.drain_to(size as usize);
                Ok(Async::Ready(ret))
            }
        }
    }
}

gen_parse!(TMethodArgsTransport, Args, parse_args, call);
gen_parse!(TMethodRetTransport, Ret, parse_ret, reply);

macro_rules! gen_serialize {
    ($stct:ident) => {
        impl <T>Serialize for $stct<T>
            where T: Se,
        {
            type In = T;

            fn serialize(&mut self, frame: Self::In, buf: &mut Vec<u8>) {
                let mut protocol = BinaryProtocol::from(buf);
                let _ = frame.serialize(&mut protocol);
            }
        }

    }
}

gen_serialize!(TMethodArgsTransport);
gen_serialize!(TMethodRetTransport);


pub type FramedThriftClientTransport<T, D, S> = EasyFramed<T, TMethodRetTransport<D>, TMethodArgsTransport<S>>;
pub type FramedThriftServerTransport<T, D, S> = EasyFramed<T, TMethodArgsTransport<D>, TMethodRetTransport<S>>;

pub fn new_thrift_client_transport<T, D, S>(inner: T) -> FramedThriftClientTransport<T, D, S>
    where T: Io,
          D: ParseThrift + FromStr + Sized,
          S: Se,
{
  EasyFramed::new(inner,
              TMethodRetTransport::new(),
              TMethodArgsTransport::new())
}

pub fn new_thrift_server_transport<T, D, S>(inner: T) -> FramedThriftServerTransport<T, D, S>
    where T: Io,
          D: ParseThrift + FromStr + Sized,
          S: Se,
{
  EasyFramed::new(inner,
              TMethodArgsTransport::new(),
              TMethodRetTransport::new())
}
