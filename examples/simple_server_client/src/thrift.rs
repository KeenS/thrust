// DO NOT EDIT: autogenerated by tokio_thrift
#![allow(dead_code, unused_imports, non_snake_case, non_camel_case_types)]
use futures::{Future, Async};
use futures::future::BoxFuture;
use tokio_thrift::protocol::{ThriftDeserializer, ThriftSerializer, ThriftMessageType};
use tokio_thrift::protocol::{Error, ThriftType, BinaryProtocol};
use tokio_thrift::protocol::{Serializer, Deserializer};
use tokio_thrift::protocol::{Deserialize, Serialize};
use tokio_thrift::transport::framed::*;
use tokio_core::reactor::Handle;
use tokio_core::net::TcpStream;
use tokio_core::io::{Codec, EasyBuf, Io, Framed};
use tokio_proto::pipeline::{ServerProto, ClientProto, Pipeline, ClientService};
use tokio_proto::{TcpServer, TcpClient};
use tokio_service::Service;

use std::io;
use std::net::SocketAddr;
use std::str::FromStr;


pub trait HelloService: Send {
    fn hello_name(&self, name: String) -> BoxFuture<String, io::Error>;
    fn hello(&self) -> BoxFuture<String, io::Error>;
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum HelloServiceMethods {
    Mhello_name,
    Mhello,
}

pub struct HelloServerCodec;

macro_rules! try_async {
    ($t:expr) => (
        match $t {
            Ok(res) => res,
            Err(Error::Byteorder(_)) => return Ok(None),
            Err(Error::Io(e)) => return Err(e),
            _ => return Err(io::Error::new(io::ErrorKind::InvalidData, "failed to parse thrift data")),
        }
    )
}


impl Codec for HelloServerCodec {
    type In = HelloServiceMethodArgs;
    type Out = HelloServiceMethodReturn;

    fn decode(&mut self, buf: &mut EasyBuf) -> Result<Option<Self::In>, io::Error> {
        let cur = io::Cursor::new(buf);
        let mut protocol = BinaryProtocol::from(cur);
        let ret = Self::In::deserialize(&mut protocol)?;
        let cur = protocol.into_inner();
        let size = cur.position();
        let buf = cur.into_inner();
        buf.drain_to(size as usize);
        Ok(Some(ret))
    }

    fn encode(&mut self, msg: Self::Out, buf: &mut Vec<u8>) -> io::Result<()> {
        let mut protocol = BinaryProtocol::from(buf);
        msg.serialize(&mut protocol).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))

    }
}


pub struct HelloServerProto;

impl<T: Io + 'static> ServerProto<T> for HelloServerProto {
    type Request = HelloServiceMethodArgs;
    type Response = HelloServiceMethodReturn;
    type Error = io::Error;
    type Transport = Framed<T, HelloServerCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(HelloServerCodec))
    }
}

#[derive(Clone)]
pub struct HelloServer<S> {
    inner: S,
}

impl <S: HelloService> HelloServer<S> {
    pub fn new(inner: S) -> Self {
        HelloServer {
            inner: inner,
        }
    }
}

impl <S: HelloService>Service for HelloServer<S> {
    type Request = HelloServiceMethodArgs;
    type Response = HelloServiceMethodReturn;
    type Error = io::Error;
    type Future = BoxFuture<HelloServiceMethodReturn, io::Error>;

    fn call(&self, req: HelloServiceMethodArgs) -> Self::Future {
        use self::HelloServiceMethodArgs::*;
        use self::HelloServiceMethodReturn::*;
        match req {
            Ahello(_args) => self.inner.hello().map(Rhello).boxed(),
            Ahello_name(_args) => self.inner.hello_name(_args.name).map(Rhello_name).boxed(),
        }
    }
}


pub struct HelloClientCodec;

macro_rules! try_async {
    ($t:expr) => (
        match $t {
            Ok(res) => res,
            Err(Error::Byteorder(_)) => return Ok(None),
            Err(Error::Io(e)) => return Err(e),
            _ => return Err(io::Error::new(io::ErrorKind::InvalidData, "failed to parse thrift data")),
        }
    )
}


impl Codec for HelloClientCodec {
    type In = HelloServiceMethodReturn;
    type Out = HelloServiceMethodArgs;

    fn decode(&mut self, buf: &mut EasyBuf) -> Result<Option<Self::In>, io::Error> {
        let cur = io::Cursor::new(buf);
        let mut protocol = BinaryProtocol::from(cur);
        let ret = Self::In::deserialize(&mut protocol)?;
        let cur = protocol.into_inner();
        let size = cur.position();
        let buf = cur.into_inner();
        buf.drain_to(size as usize);
        Ok(Some(ret))
    }

    fn encode(&mut self, msg: Self::Out, buf: &mut Vec<u8>) -> io::Result<()> {
        let mut protocol = BinaryProtocol::from(buf);
        msg.serialize(&mut protocol).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))

    }
}


pub struct HelloClientProto;

impl<T: Io + 'static> ClientProto<T> for HelloClientProto {
    type Request = HelloServiceMethodArgs;
    type Response = HelloServiceMethodReturn;
    type Error = io::Error;
    type Transport = Framed<T, HelloClientCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(HelloClientCodec))
    }
}


pub struct HelloClient<T: 'static+Io> {
    client: ClientService<T, HelloClientProto>,
}

impl <T: 'static+Io>HelloClient<T> {
    pub fn new(client: ClientService<T, HelloClientProto>) -> Self {
        HelloClient {
            client: client,
        }
    }
}

impl <T: 'static+Io>HelloService for HelloClient<T> {
    fn hello_name(&self, name: String) -> BoxFuture<String, io::Error> {
        use self::HelloServiceMethodArgs::*;
        use self::HelloServiceMethodReturn::*;
        self.client.call(Ahello_name(Hellohello_nameArgs{name: name})).and_then(|ret| match ret {
            Rhello_name(s) => Ok(s),
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "internal protocol error"))
        }
        ).boxed()
    }

    fn hello(&self) -> BoxFuture<String, io::Error> {
        use self::HelloServiceMethodArgs::*;
        use self::HelloServiceMethodReturn::*;
        self.client.call(Ahello(HellohelloArgs{})).and_then(|ret| match ret {
            Rhello(s) => Ok(s),
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "internal protocol error"))
        }
        ).boxed()
    }
}


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum HelloServiceMethodArgs {
    Ahello_name(Hellohello_nameArgs),
    Ahello(HellohelloArgs),
}


impl Serialize for HelloServiceMethodArgs {
    fn serialize<S>(&self, s: &mut S) -> Result<(), Error>
        where S: Serializer + ThriftSerializer
    {
        use self::HelloServiceMethodArgs::*;
        match self {
            &Ahello_name(ref b) => {
                try!(s.write_message_begin("hello_name", ThriftMessageType::Call));
                try!(b.serialize(s));
                try!(s.write_message_end());
            },
            &Ahello(ref b) => {
                try!(s.write_message_begin("hello", ThriftMessageType::Call));
                try!(b.serialize(s));
                try!(s.write_message_end());
            },
        };
        Ok(())
    }
}

impl Deserialize for HelloServiceMethodArgs {
    fn deserialize<D>(de: &mut D) -> Result<Self, Error>
        where D: Deserializer + ThriftDeserializer,
    {
        let msg = try!(de.read_message_begin());
        //assert!(msg.type) == $msg_type
        let ret = match msg.name.as_ref() {
            "hello_name" => HelloServiceMethodArgs::Ahello_name(Hellohello_nameArgs::deserialize(de)?),
            "hello" => HelloServiceMethodArgs::Ahello(HellohelloArgs::deserialize(de)?),
            _ => return Err(Error::from(io::Error::new(io::ErrorKind::InvalidData, "failed to parse thrift data"))),
        };
        let _ = try!(de.read_message_end());
        Ok(ret)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum HelloServiceMethodReturn {
    Rhello_name(String),
    Rhello(String),

}

impl Serialize for HelloServiceMethodReturn {
    fn serialize<S>(&self, s: &mut S) -> Result<(), Error>
        where S: Serializer + ThriftSerializer
    {
        use self::HelloServiceMethodReturn::*;
        match self {
            &Rhello_name(ref b) => {
                try!(s.write_message_begin("hello_name", ThriftMessageType::Reply));
                try!(b.serialize(s));
                try!(s.write_message_end());
            },
            &Rhello(ref b) => {
                try!(s.write_message_begin("hello", ThriftMessageType::Reply));
                try!(b.serialize(s));
                try!(s.write_message_end());
            },
        };
        Ok(())
    }
}

impl Deserialize for HelloServiceMethodReturn {
    fn deserialize<D>(de: &mut D) -> Result<Self, Error>
        where D: Deserializer + ThriftDeserializer,
    {
        let msg = try!(de.read_message_begin());
        //assert!(msg.type) == $msg_type
        let ret = match msg.name.as_ref() {
            "hello_name" => HelloServiceMethodReturn::Rhello_name(String::deserialize(de)?),
            "hello" => HelloServiceMethodReturn::Rhello(String::deserialize(de)?),
            _ => return Err(Error::from(io::Error::new(io::ErrorKind::InvalidData, "failed to parse thrift data"))),
        };
        let _ = try!(de.read_message_end());
        Ok(ret)
    }
}


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Hellohello_nameArgs {
    pub name: String,
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct HellohelloArgs {
}

impl Serialize for Hellohello_nameArgs {
    fn serialize<S>(&self, s: &mut S) -> Result<(), Error>
        where S: Serializer + ThriftSerializer
    {
        try!(s.write_struct_begin("Hello_hello_name_Args"));
        try!(s.write_field_begin("name", ThriftType::String, 1));
        try!(self.name.serialize(s));
        try!(s.write_field_end());
        try!(s.write_field_stop());
        try!(s.write_struct_end());
        Ok(())
    }
}
impl Serialize for HellohelloArgs {
    fn serialize<S>(&self, s: &mut S) -> Result<(), Error>
        where S: Serializer + ThriftSerializer
    {
        try!(s.write_struct_begin("Hello_hello_Args"));
        try!(s.write_field_stop());
        try!(s.write_struct_end());
        Ok(())
    }
}


impl Deserialize for Hellohello_nameArgs {
    fn deserialize<D>(de: &mut D) -> Result<Self, Error>
        where D: Deserializer + ThriftDeserializer,
    {
        try!(de.read_struct_begin());
        let mut name = None;
        loop {
            let scheme_field = try!(de.read_field_begin());
            if scheme_field.ty == ThriftType::Stop {
                break;
            };
            match scheme_field.seq {
                1 => {
                    if scheme_field.ty == ThriftType::String {
                        name = Some(try!(de.deserialize_str()));
                    } else {
                        // skip
                    }
                },
                _ => (),// skip
            }
            try!(de.read_field_end());
        };
        try!(de.read_struct_end());
        let args = Hellohello_nameArgs {
            name: name.unwrap(),
        };
        Ok(args)
    }
}

impl Deserialize for HellohelloArgs {
    fn deserialize<D>(de: &mut D) -> Result<Self, Error>
        where D: Deserializer + ThriftDeserializer,
    {
        try!(de.read_struct_begin());

        loop {
            let scheme_field = try!(de.read_field_begin());
            if scheme_field.ty == ThriftType::Stop {
                break;
            };
            match scheme_field.seq {

                _ => (),// skip
            }
            try!(de.read_field_end());
        };
        try!(de.read_struct_end());
        let args = HellohelloArgs {

        };
        Ok(args)
    }
}

