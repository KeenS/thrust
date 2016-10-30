use protocol::*;
use binary_protocol::*;

pub fn create_empty_thrift_message(method: &str, ty: ThriftMessageType) -> Vec<u8> {
    let mut buf:Vec<u8> = Vec::new();
    let mut se = BinaryProtocol::new(buf);
    se.write_message_begin(method, ty);
    se.write_message_end();
    se.into_inner()
}

