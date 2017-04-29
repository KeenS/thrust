use super::{Serializer, Deserializer, ThriftSerializer, ThriftField, ThriftMessage, ThriftDeserializer, ThriftMessageType, ThriftType, Error};
use transport::{VoidTransport, ReadTransport, WriteTransport};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::iter;

#[allow(overflowing_literals)]
pub const THRIFT_VERSION_1: i32 = 0x80010000;
#[allow(overflowing_literals)]
pub const THRIFT_VERSION_MASK: i32 = 0xffff0000;
#[allow(overflowing_literals)]
pub const THRIFT_TYPE_MASK: i32 = 0x000000ff;

pub struct BinaryProtocol<T>{
    inner: T
}

impl <T>BinaryProtocol<T> {
    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl <T: VoidTransport>BinaryProtocol<T> {
    pub fn new(inner: T) -> Self {
        BinaryProtocol{
            inner: inner,
        }
    }
}

impl <T: VoidTransport>From<T> for BinaryProtocol<T> {
    fn from(w: T) -> Self {
        Self::new(w)
    }
}

impl <T: WriteTransport> Serializer for BinaryProtocol<T> {

    fn serialize_bool(&mut self, val: bool) -> Result<(), Error> {
        if val {
            self.serialize_i8(1)
        } else {
            self.serialize_i8(0)
        }
    }

    fn serialize_usize(&mut self, val: usize) -> Result<(), Error> {
        self.serialize_isize(val as isize)
    }

    fn serialize_isize(&mut self, val: isize) -> Result<(), Error> {
        self.serialize_i64(val as i64)
    }

    fn serialize_u64(&mut self, val: u64) -> Result<(), Error> {
        self.serialize_i64(val as i64)
    }

    fn serialize_i64(&mut self, val: i64) -> Result<(), Error> {
        try!(self.inner.write_i64::<BigEndian>(val));
        Ok(())
    }

    fn serialize_u32(&mut self, val: u32) -> Result<(), Error> {
        self.serialize_i32(val as i32)
    }

    fn serialize_i32(&mut self, val: i32) -> Result<(), Error> {
        try!(self.inner.write_i32::<BigEndian>(val));
        Ok(())
    }

    fn serialize_u16(&mut self, val: u16) -> Result<(), Error> {
        self.serialize_i16(val as i16)
    }

    fn serialize_i16(&mut self, val: i16) -> Result<(), Error> {
        try!(self.inner.write_i16::<BigEndian>(val));
        Ok(())
    }

    fn serialize_u8(&mut self, val: u8) -> Result<(), Error> {
        self.serialize_i8(val as i8)
    }

    fn serialize_i8(&mut self, val: i8) -> Result<(), Error> {
        try!(self.inner.write_i8(val));
        Ok(())
    }

    fn serialize_f64(&mut self, val: f64) -> Result<(), Error> {
        try!(self.inner.write_f64::<BigEndian>(val));
        Ok(())
    }

    fn serialize_bytes(&mut self, val: &[u8]) -> Result<(), Error> {
        try!(self.serialize_i32(val.len() as i32));
        try!(self.inner.write(val));
        Ok(())
    }

    fn serialize_str(&mut self, val: &str) -> Result<(), Error> {
        self.serialize_bytes(val.as_bytes())
    }

    fn serialize_string(&mut self, val: String) -> Result<(), Error> {
        self.serialize_str(&*val)
    }
}

impl <T: WriteTransport>ThriftSerializer for BinaryProtocol<T> {
    fn write_message_begin(&mut self, name: &str, message_type: ThriftMessageType) -> Result<(), Error> {
        let version = THRIFT_VERSION_1 | message_type as i32;

        try!(self.serialize_i32(version));
        try!(self.serialize_str(name));
        try!(self.serialize_i16(0));

        Ok(())
    }

    fn write_struct_begin(&mut self, _name: &str) -> Result<(), Error> {
        Ok(())
    }

    fn write_struct_end(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn write_field_begin(&mut self, _name: &str, ty: ThriftType, id: i16) -> Result<(), Error> {
        try!(self.serialize_i8(ty as i8));
        try!(self.serialize_i16(id));
        Ok(())
    }

    fn write_field_end(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn write_field_stop(&mut self) -> Result<(), Error> {
        try!(self.serialize_i8(ThriftType::Stop as i8));
        Ok(())
    }

    fn write_message_end(&mut self) -> Result<(), Error> {
        Ok(())
    }
}



impl<T: ReadTransport> Deserializer for BinaryProtocol<T> {
    fn deserialize_bool(&mut self) -> Result<bool, Error> {
        Ok(try!(self.inner.read_i8()) != 0)
    }

    fn deserialize_usize(&mut self) -> Result<usize, Error> {
        Ok(try!(self.deserialize_isize()) as usize)
    }

    fn deserialize_isize(&mut self) -> Result<isize, Error> {
        Ok(try!(self.deserialize_i64()) as isize)
    }

    fn deserialize_u64(&mut self) -> Result<u64, Error> {
        Ok(try!(self.deserialize_i64()) as u64)
    }

    fn deserialize_i64(&mut self) -> Result<i64, Error> {
        Ok(try!(self.inner.read_i64::<BigEndian>()))
    }

    fn deserialize_u32(&mut self) -> Result<u32, Error> {
        Ok(try!(self.deserialize_i32()) as u32)
    }

    fn deserialize_i32(&mut self) -> Result<i32, Error> {
        Ok(try!(self.inner.read_i32::<BigEndian>()))
    }

    fn deserialize_u16(&mut self) -> Result<u16, Error> {
        Ok(try!(self.deserialize_i16()) as u16)
    }

    fn deserialize_i16(&mut self) -> Result<i16, Error> {
        Ok(try!(self.inner.read_i16::<BigEndian>()))
    }

    fn deserialize_u8(&mut self) -> Result<u8, Error> {
        Ok(try!(self.deserialize_i8()) as u8)
    }

    fn deserialize_i8(&mut self) -> Result<i8, Error> {
        Ok(try!(self.inner.read_i8()))
    }

    fn deserialize_f64(&mut self) -> Result<f64, Error> {
        Ok(try!(self.inner.read_f64::<BigEndian>()))
    }


    fn deserialize_bytes(&mut self) -> Result<Vec<u8>, Error> {
        let len = try!(self.deserialize_i32()) as usize;
        let mut buf = Vec::with_capacity(len);

        buf.extend(iter::repeat(0).take(len));
        try!(self.inner.read(&mut buf));

        Ok(buf)
    }

    fn deserialize_str(&mut self) -> Result<String, Error> {
        let buf = try!(self.deserialize_bytes());
        let s = try!(String::from_utf8(buf));
        Ok(s)
    }
}

impl<T: ReadTransport> ThriftDeserializer for BinaryProtocol<T> {
    fn read_message_begin(&mut self) -> Result<ThriftMessage, Error> {
        let size: i32 = try!(self.deserialize_i32());

        if size < 0 {
            let version = size & THRIFT_VERSION_MASK;
            if version != THRIFT_VERSION_1 {
                Err(Error::BadVersion)
            } else {
                Ok(ThriftMessage {
                    name: try!(self.deserialize_str()),
                    ty: ThriftMessageType::from((size & THRIFT_TYPE_MASK) as i8),
                    seq: try!(self.deserialize_i16())
                })
            }
        } else {
            Err(Error::ProtocolVersionMissing)
        }
    }

    fn read_message_end(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn read_struct_begin(&mut self) -> Result<String, Error> {
        Ok("".to_string())
    }

    fn read_struct_end(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn read_field_begin(&mut self) -> Result<ThriftField, Error> {
        let mut field = ThriftField {
            name: None,
            ty: ThriftType::from(try!(self.deserialize_i8())),
            seq: 0
        };

        if field.ty == ThriftType::Stop {
            Ok(field)
        } else {
            field.seq = try!(self.deserialize_i16());
            Ok(field)
        }
    }

    fn read_field_end(&mut self) -> Result<(), Error> {
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use std::io::{Cursor, Read, Write};
    use byteorder::{ReadBytesExt, WriteBytesExt, BigEndian};
    use protocol::{ThriftMessageType, ThriftType, ThriftMessage, ThriftDeserializer, ThriftSerializer, Serializer, Serialize, Deserializer};
    use super::*;
    use ::protocol::Deserialize;


    #[test]
    fn deserialize_bool() {
        let mut de = BinaryProtocol::new(Cursor::new(vec![1u8]));
        let val: bool = Deserialize::deserialize(&mut de).unwrap();
        assert_eq!(val, true);
    }

    #[test]
    fn deserialize_u16() {
        let mut buf = Vec::new();
        buf.write_u16::<BigEndian>(32000);
        let mut de = BinaryProtocol::new(Cursor::new(buf));
        let val: u16 = Deserialize::deserialize(&mut de).unwrap();
        assert_eq!(val, 32000);
    }

    #[test]
    fn deserialize_i16() {
        let mut buf = Vec::new();
        buf.write_i16::<BigEndian>(-32000);
        let mut de = BinaryProtocol::new(Cursor::new(buf));
        let val: i16 = Deserialize::deserialize(&mut de).unwrap();
        assert_eq!(val, -32000);
    }

    #[test]
    fn deserialize_u32() {
        let mut buf = Vec::new();
        buf.write_u32::<BigEndian>(32000);
        let mut de = BinaryProtocol::new(Cursor::new(buf));
        let val: u32 = Deserialize::deserialize(&mut de).unwrap();
        assert_eq!(val, 32000);
    }

    #[test]
    fn deserialize_i32() {
        let mut buf = Vec::new();
        buf.write_i32::<BigEndian>(32000);
        let mut de = BinaryProtocol::new(Cursor::new(buf));
        let val: i32 = Deserialize::deserialize(&mut de).unwrap();
        assert_eq!(val, 32000);
    }

    #[test]
    fn deserialize_u64() {
        let mut buf = Vec::new();
        buf.write_u64::<BigEndian>(32000);
        let mut de = BinaryProtocol::new(Cursor::new(buf));
        let val: u64 = Deserialize::deserialize(&mut de).unwrap();
        assert_eq!(val, 32000);
    }

    #[test]
    fn deserialize_i64() {
        let mut buf = Vec::new();
        buf.write_i64::<BigEndian>(32000);
        let mut de = BinaryProtocol::new(Cursor::new(buf));
        let val: i64 = Deserialize::deserialize(&mut de).unwrap();
        assert_eq!(val, 32000);
    }

    #[test]
    fn deserialize_f64() {
        let mut buf = Vec::new();
        buf.write_f64::<BigEndian>(32000.0);
        let mut de = BinaryProtocol::new(Cursor::new(buf));
        let val: f64 = Deserialize::deserialize(&mut de).unwrap();
        assert_eq!(val, 32000.0);
    }

    #[test]
    fn deserialize_string() {
        let mut buf = Vec::new();
        let i = "foobar";
        buf.write_i32::<BigEndian>(i.len() as i32);
        buf.write(i.as_bytes());
        let mut de = BinaryProtocol::new(Cursor::new(buf));
        let val: String = Deserialize::deserialize(&mut de).unwrap();
        assert_eq!(&*val, "foobar");
    }

   #[test]
    fn serialize_bool_true() {
        let mut v: Vec<u8> = Vec::new();
        {
            let mut s = BinaryProtocol::new(&mut v);
            s.serialize_bool(true);
        }

        assert_eq!(v[0], 1);
    }

    #[test]
    fn serialize_bool_false() {
        let mut v = Vec::new();
        {
            let mut s = BinaryProtocol::new(&mut v);
            s.serialize_bool(false);
        }

        assert_eq!(v[0], 0);
    }

    #[test]
    fn serialize_i8() {
        let mut v = Vec::new();
        {
            let mut s = BinaryProtocol::new(&mut v);
            s.serialize_i8(5);
        }

        assert_eq!(v[0], 5);
    }

    #[test]
    fn serialize_i8_neg() {
        let mut v = Vec::new();
        {
            let mut s = BinaryProtocol::new(&mut v);
            s.serialize_i8(-5);
        }

        assert_eq!(v[0] as i8, -5);
    }

    #[test]
    fn serialize_i16() {
        let mut v = Vec::new();
        {
            let mut s = BinaryProtocol::new(&mut v);
            s.serialize_i16(900);
        }

        let mut cursor = Cursor::new(v);
        assert_eq!(900, cursor.read_i16::<BigEndian>().unwrap());
    }

    #[test]
    fn serialize_i16_neg() {
        let mut v = Vec::new();
        {
            let mut s = BinaryProtocol::new(&mut v);
            s.serialize_i16(-900);
        }

        let mut cursor = Cursor::new(v);
        assert_eq!(-900, cursor.read_i16::<BigEndian>().unwrap());
    }

    #[test]
    fn serialize_i32() {
        let mut v = Vec::new();
        {
            let mut s = BinaryProtocol::new(&mut v);
            s.serialize_i32(3000000);
        }

        let mut cursor = Cursor::new(v);
        assert_eq!(3000000, cursor.read_i32::<BigEndian>().unwrap());
    }

    #[test]
    fn serialize_i32_neg() {
        let mut v = Vec::new();
        {
            let mut s = BinaryProtocol::new(&mut v);
            s.serialize_i32(-3000000);
        }

        let mut cursor = Cursor::new(v);
        assert_eq!(-3000000, cursor.read_i32::<BigEndian>().unwrap());
    }

    #[test]
    fn serialize_i64() {
        let mut v = Vec::new();
        {
            let mut s = BinaryProtocol::new(&mut v);
            s.serialize_i64(33000000);
        }

        let mut cursor = Cursor::new(v);
        assert_eq!(33000000, cursor.read_i64::<BigEndian>().unwrap());
    }

    #[test]
    fn serialize_i64_neg() {
        let mut v = Vec::new();
        {
            let mut s = BinaryProtocol::new(&mut v);
            s.serialize_i64(-33000000);
        }

        let mut cursor = Cursor::new(v);
        assert_eq!(-33000000, cursor.read_i64::<BigEndian>().unwrap());
    }

    #[test]
    fn protocol_begin() {
        let mut v = Vec::new();
        {
            let mut proto = BinaryProtocol::new(&mut v);
            proto.write_message_begin("foobar", ThriftMessageType::Call);
        }

        let mut cursor = Cursor::new(v);
        let version = THRIFT_VERSION_1 | ThriftMessageType::Call as i32;

        assert_eq!(version, cursor.read_i32::<BigEndian>().unwrap());
        // XXX Decode string and seqid.
    }

    #[test]
    fn write_and_read_message_begin() {
        let mut buf = Vec::new();

        {
            let mut se = BinaryProtocol::new(&mut buf);
            se.write_message_begin("Foobar123", ThriftMessageType::Call);
        }

        let mut de = BinaryProtocol::new(Cursor::new(buf));
        let msg = de.read_message_begin().unwrap();

        assert_eq!(msg.name, "Foobar123");
        assert_eq!(msg.ty, ThriftMessageType::Call);
    }
}
