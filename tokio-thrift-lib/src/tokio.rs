use std::marker::PhantomData;
use tokio_core::io::{Codec, EasyBuf, Io, Framed};
use tokio_proto::pipeline::{ServerProto, ClientProto, Pipeline, ClientService};
use tokio_proto::{TcpServer, TcpClient};
use std::io;
use protocol::{Error, ThriftType, BinaryProtocol};
use protocol::{Deserialize, Serialize};
use std::net::SocketAddr;


pub struct ThriftCodec<In, Out>(PhantomData<In>, PhantomData<Out>);

impl<In, Out> ThriftCodec<In, Out> {
    pub fn new() -> Self {
        ThriftCodec(PhantomData, PhantomData)
    }
}

impl<In: Deserialize, Out: Serialize> Codec for ThriftCodec<In, Out> {
    type In = In;
    type Out = Out;

    fn decode(&mut self, buf: &mut EasyBuf) -> Result<Option<Self::In>, io::Error> {
        let cur = io::Cursor::new(buf);
        let mut protocol = BinaryProtocol::from(cur);
        let ret = match Self::In::deserialize(&mut protocol) {
            Ok(ret) => ret,
            Err(Error::EOF) => return Ok(None),
            Err(e) => return Err(io::Error::from(e)),
        };
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


pub struct ThriftProto<Req, Res>(PhantomData<Req>, PhantomData<Res>);

impl<Req, Res> ThriftProto<Req, Res> {
    pub fn new() -> Self {
        ThriftProto(PhantomData, PhantomData)
    }
}


impl<Req: Serialize + 'static, Res: Deserialize + 'static, T: Io + 'static> ClientProto<T>
    for ThriftProto<Req, Res> {
    type Request = Req;
    type Response = Res;
    type Transport = Framed<T, ThriftCodec<Res, Req>>;
    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(ThriftCodec::<Res, Req>::new()))
    }
}

impl<Req: Deserialize + 'static, Res: Serialize + 'static, T: Io + 'static> ServerProto<T>
    for ThriftProto<Req, Res> {
    type Request = Req;
    type Response = Res;
    type Transport = Framed<T, ThriftCodec<Req, Res>>;
    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(ThriftCodec::new()))
    }
}


pub fn new_tcp_client<Req: Serialize + 'static, Res: Deserialize + 'static>
    ()
    -> TcpClient<Pipeline, ThriftProto<Req, Res>>
{
    TcpClient::new(ThriftProto::<Req, Res>::new())
}

pub fn new_tcp_server<Req: Deserialize + Send + Sync + 'static,
                      Res: Serialize + Send + Sync + 'static>
    (addr: SocketAddr)
     -> TcpServer<Pipeline, ThriftProto<Req, Res>> {
    TcpServer::new(ThriftProto::<Req, Res>::new(), addr)
}
