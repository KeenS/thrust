pub mod framed_transport;
pub use self::{framed_transport as framed};

use std::io::{self, Write, Read, Cursor};


pub trait VoidTransport {
    fn open(&mut self) -> io::Result<()>;
    fn close(&mut self) -> io::Result<()>;
}

pub trait ReadTransport: VoidTransport + Read {
}

pub trait WriteTransport: VoidTransport + Write {
}

pub trait Transport: ReadTransport + WriteTransport {
}

impl <T>WriteTransport for T
    where T: VoidTransport + Write {
}

impl <T>ReadTransport for T
    where T: VoidTransport + Read {
}


macro_rules! impl_void_transport {
    ($ty: ty) => {
        impl VoidTransport for $ty {
            fn open(&mut self) -> io::Result<()> {
                Ok(())
            }
            fn close(&mut self) -> io::Result<()> {
                Ok(())
            }
        }
    };
    ($ty: ty, $param:tt) => {
        impl <$param>VoidTransport for $ty {
            fn open(&mut self) -> io::Result<()> {
                Ok(())
            }
            fn close(&mut self) -> io::Result<()> {
                Ok(())
            }
        }
    };

    ($ty: ty, $param:tt $(, $params:tt)*) => {
        impl <$param, $($params)*>VoidTransport for $ty {
            fn open(&mut self) -> io::Result<()> {
                Ok(())
            }
            fn close(&mut self) -> io::Result<()> {
                Ok(())
            }
        }
    };

    ($ty: ty,  where $t:ident : $bound:tt $($bounds:tt)*) => {
        impl <$t>VoidTransport for $ty
            where $t: $bound$($bounds)* {
            fn open(&mut self) -> io::Result<()> {
                Ok(())
            }
            fn close(&mut self) -> io::Result<()> {
                Ok(())
            }
        }
    };
}

impl_void_transport!(Vec<u8>);
impl_void_transport!(&'a mut Vec<u8>, 'a);
impl_void_transport!(&'a [u8], 'a);
impl_void_transport!(&'a mut [u8], 'a);
impl_void_transport!(Cursor<T>, where T: AsRef<[u8]>);

