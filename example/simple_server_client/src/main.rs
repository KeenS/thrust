extern crate futures;
extern crate tokio_core as tokio;
extern crate tokio_service as service;
extern crate tokio_proto as proto;
extern crate simple_server_client;

use std::io;
use futures::{Future, done};
use tokio::reactor::Core;
use tokio::net::TcpStream;
use simple_server_client::thrift::*;

#[derive(Clone)]
struct HelloServerImpl;

// implement HelloService
impl HelloService for HelloServerImpl {
    fn hello_name(&self, name: String) -> Box<Future<Item = String, Error = io::Error>> {
        println!("GOT: {:?}", name);
        Box::new(done(Ok(format!("Hello, {}", name))))
    }

    fn hello(&self) -> Box<Future<Item = String, Error = io::Error>> {
        println!("CALLED");
        Box::new(done(Ok(format!("Hello, World"))))
    }
}

pub fn main() {
    let mut core = Core::new().unwrap();

    // This brings up our server.
    let addr = "127.0.0.1:12345".parse().unwrap();

    // instanciate and start the server.
    let server = HelloServer::new(HelloServerImpl);
    server.serve(&core.handle(), addr,).unwrap();

    // Now our client. We use the same core as for the server - usually though this would be
    // done in a separate program most likely on a separate machine.
    let handle = core.handle().clone();
    let stream = core.run(TcpStream::connect(&addr, &handle.clone())).expect("connection failed");
    let client = HelloClient::new(&handle.clone(), stream);


    let resp = client.hello_name("keen".to_string());
    let resp = core.run(resp).expect("rpc failed");
    println!("RESPONSE: {:?}", resp);
    let resp = client.hello();
    let resp = core.run(resp).expect("rpc failed");
    println!("RESPONSE: {:?}", resp);

}
