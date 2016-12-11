extern crate futures;
extern crate tokio_core as tokio;
extern crate tokio_service as service;
extern crate tokio_proto as proto;
extern crate simple_server_client;

use futures::{finished, failed, Future};
use futures::future::BoxFuture;
use proto::TcpServer;
use simple_server_client::thrift::*;

#[derive(Clone)]
struct HelloServerImpl;

// implement HelloService
impl HelloService for HelloServerImpl {
    fn hello_name(&self, name: String) -> BoxFuture<String, String> {
        println!("GOT: {:?}", name);
        if name == "error".as_ref() {
            failed::<String, _>("error passed".to_string()).boxed()
        } else {
            finished::<_, String>(format!("Hello, {}", name)).boxed()
        }

    }

    fn hello(&self) -> BoxFuture<String, ()> {
        println!("CALLED");
        finished::<_, ()>(format!("Hello, World")).boxed()
    }
}

pub fn main() {

    // This brings up our server.
    let addr = "127.0.0.1:12345".parse().unwrap();

    // instanciate and start the server.
    let server = TcpServer::new(HelloServerProto, addr);
    server.serve(|| Ok(HelloServer::new(HelloServerImpl)))
}
