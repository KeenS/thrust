use std::io::{self, Cursor};
use tokio_core::io::Io;
use tokio_core::easy::{Parse, Serialize, EasyBuf, EasyFramed};
use thrust::protocol::{Deserialize as De, Serialize as Se, Error};
use thrust::binary_protocol::BinaryProtocol;
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

// pub struct TTransport;

// impl TTransport {
//     pub fn new() -> Self {
//         TTransport
//     }
// }

//impl <D>Parse for TTransport<D>
//    where D: Deserializer + ThriftDeserializer + From<ReadTransport>,
impl <T>Parse for TTransport<T>
    where T: De
{
    type Out = T;

    fn parse(&mut self, buf: &mut EasyBuf) -> Poll<Self::Out, io::Error> {
        let cur = Cursor::new(buf);
        let mut protocol = BinaryProtocol::from(cur);
        let ret = match De::deserialize(&mut protocol) {
            Ok(res) => Ok(Async::Ready(res)),
            Err(Error::Byteorder(_)) => Ok(Async::NotReady),
            Err(Error::Io(e)) => Err(e),
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "failed to parse thrift data"))
        };
        let cur = protocol.into_inner();
        let size = cur.position();
        let buf = cur.into_inner();
        buf.drain_to(size as usize);
        ret
    }
}

//impl <S>Serialize for TTransport<S>
//    where S: Serializer + ThriftSerializer + From<WriteTransport>,
impl <T>Serialize for TTransport<T>
    where T: Se
{
    type In = T;

    fn serialize(&mut self, frame: Self::In, buf: &mut Vec<u8>) {
        let mut protocol = BinaryProtocol::from(buf);
        let _ = frame.serialize(&mut protocol);
    }
}


pub type FramedThriftTransport<T, D, S> = EasyFramed<T, TTransport<D>, TTransport<S>>;

pub fn new_thrift_transport<T, D, S>(inner: T) -> FramedThriftTransport<T, D, S>
    where T: Io,
          D: De,
          S: Se,
          // D: Deserializer + ThriftDeserializer + Parse,
          // S: Serializer + ThriftSerializer + Serialize,
{
  EasyFramed::new(inner,
              TTransport::new(),
              TTransport::new())
}
