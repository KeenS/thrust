extern crate futures;
extern crate tokio_core as tokio;
extern crate tokio_service as service;
extern crate tokio_proto as proto;
extern crate simple_server_client;
extern crate tokio_thrift;

use futures::Future;
use tokio::reactor::Core;
use simple_server_client::thrift::*;
use tokio_thrift::tokio::new_tcp_client;


pub fn main() {

    // This brings up our server.
    let addr = "127.0.0.1:12345".parse().unwrap();

    let mut core = Core::new().unwrap();

    let client = new_tcp_client();
    let client = client.connect(&addr, &core.handle())
        .map(HelloClient::new);
    let client = core.run(client).expect("failed to connect the server");

    // the client implements `HelloService` so you can call the methods directly
    let hello_ret = client.hello();
    let hello_name_ret1 = client.hello_name("keen".to_string());
    let hello_name_ret2 = client.hello_name("error".to_string());

    let hello_ret = core.run(hello_ret).expect("rpc failed");
    let hello_name_ret1 = core.run(hello_name_ret1).expect("rpc failed");
    let hello_name_ret2 = core.run(hello_name_ret2).expect("rpc failed");

    println!("RESPONSE: {:?}", hello_ret);
    println!("RESPONSE: {:?}", hello_name_ret1);
    println!("RESPONSE: {:?}", hello_name_ret2);
}
