extern crate futures;
extern crate tokio_core as tokio;
extern crate tokio_service as service;
extern crate tokio_proto as proto;
extern crate simple_server_client;

use std::io;
use std::thread::{spawn, sleep};
use std::time;
use futures::{Future, done};
use futures::future::BoxFuture;
use tokio::reactor::Core;
use proto::{TcpServer, TcpClient};
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

    // since server.serve blocks, spawn a new thread and won't wait for it terminate
    let _handle = spawn(move || {
        // instanciate and start the server.
        let server = TcpServer::new(HelloServerProto, addr);
        server.serve(|| Ok(HelloServer::new(HelloServerImpl)))
    });

    let mut core = Core::new().unwrap();
    // Now our client. We use the same core as for the server - usually though this would be
    // done in a separate program most likely on a separate machine.
    let client =  TcpClient::new(HelloClientProto);
    let hund_millis = time::Duration::from_millis(100);
    let hello_client;
    // just need a label, not looping
    'client: loop {
        for _ in 0..10 {
            let client = client.connect(&addr, &core.handle())
                .map(HelloClient::new);
            match core.run(client) {
                Ok(c) => {
                    hello_client = c;
                    break 'client;
                },
                Err(_) => {
                    sleep(hund_millis);
                },
            }
        }
        panic!("failed to connect to the server");
    }

    let hello_ret = hello_client.hello();
    let hello_name_ret = hello_client.hello_name("keen".to_string());

    let hello_ret = core.run(hello_ret).expect("rpc failed");
    let hello_name_ret = core.run(hello_name_ret).expect("rpc failed");

    println!("RESPONSE: {:?}", hello_ret);
    println!("RESPONSE: {:?}", hello_name_ret);
}
