extern crate futures;
extern crate tokio_core as tokio;
extern crate tokio_service as service;
extern crate tokio_proto as proto;
extern crate simple_server_client;

use std::io;
use futures::done;
use futures::future::BoxFuture;
use proto::TcpServer;
use simple_server_client::thrift::*;

#[derive(Clone)]
struct HelloServerImpl;

// implement HelloService
impl HelloService for HelloServerImpl {
    fn hello_name(&self, name: String) -> BoxFuture<String, io::Error> {
        println!("GOT: {:?}", name);
        Box::new(done(Ok(format!("Hello, {}", name))))
    }

    fn hello(&self) -> BoxFuture<String, io::Error> {
        println!("CALLED");
        Box::new(done(Ok(format!("Hello, World"))))
    }
}

pub fn main() {

    // This brings up our server.
    let addr = "127.0.0.1:12345".parse().unwrap();

    // instanciate and start the server.
    let server = TcpServer::new(HelloServerProto, addr);
    server.serve(|| Ok(HelloServer::new(HelloServerImpl)))
}
