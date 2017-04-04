extern crate futures;
extern crate tokio_core as tokio;
extern crate tokio_service as service;
extern crate tokio_proto as proto;
extern crate simple_server_client;
extern crate tokio_thrift;

use futures::{finished, failed, Future};
use futures::future::BoxFuture;
use tokio_thrift::tokio::new_tcp_server;
use simple_server_client::thrift::*;

#[derive(Clone)]
struct HelloServerImpl;

// implement HelloService
impl HelloService for HelloServerImpl {
    fn hello_name(&self, name: String) -> BoxFuture<String, ()> {
        println!("GOT: {:?}", name);
        if name == "error".as_ref() {
            failed::<String, _>(()).boxed()
        } else {
            finished::<_, ()>(format!("Hello, {}", name)).boxed()
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
    let server = new_tcp_server::<_, HelloServiceMethodReturn>(addr);
    server.serve(|| Ok(HelloServer::new(HelloServerImpl)))
}
