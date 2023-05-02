use std::io::{self, Read, Write};

pub trait ReadBytes<const COUNT: usize> {
    fn read_bytes(&mut self) -> io::Result<[u8; COUNT]>;
}

pub trait WriteBytes<const COUNT: usize> {
    fn write_bytes(&mut self, b: [u8; COUNT]) -> io::Result<()>;
}

impl<const COUNT: usize, R: Read> ReadBytes<COUNT> for R {
    fn read_bytes(&mut self) -> io::Result<[u8; COUNT]> {
        let mut buf = [0u8; COUNT];
        self.read_exact(&mut buf)?;
        Ok(buf)
    }
}

impl<const COUNT: usize, W: Write> WriteBytes<COUNT> for W {
    fn write_bytes(&mut self, b: [u8; COUNT]) -> io::Result<()> {
        self.write_all(&b)
    }
}

pub trait ReadType<T> {
    fn read_type(&mut self) -> io::Result<T>;
}

pub trait WriteType<T> {
    fn write_type(&mut self, t: T) -> io::Result<()>;
}

macro_rules! impl_rw {
    ($($t:ty),*) => {
        $(
            impl<R: ReadBytes<{std::mem::size_of::<$t>()}>> ReadType<$t> for R {
                fn read_type(&mut self) -> io::Result<$t> {
                    Ok(<$t>::from_be_bytes(self.read_bytes()?))
                }
            }
            
            impl<W: WriteBytes<{std::mem::size_of::<$t>()}>> WriteType<$t> for W {
                fn write_type(&mut self, t: $t) -> io::Result<()> {
                    self.write_bytes(t.to_be_bytes())
                }
            }
        )*
    };
}

impl_rw!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64);

impl<R: ReadType<u8>> ReadType<bool> for R {
    fn read_type(&mut self) -> io::Result<bool> {
        Ok(self.read_type()? != 0)
    }
}

impl<W: WriteType<u8> + WriteType<u8>> WriteType<bool> for W {
    fn write_type(&mut self, t: bool) -> io::Result<()> {
        self.write_type(if t { 1 } else { 0 })
    }
}

impl<R: ReadType<u32> + ReadType<u8>> ReadType<String> for R {
    fn read_type(&mut self) -> io::Result<String> {
        let length: u32 = self.read_type()?;
        let mut v = Vec::new();
        for _ in 0..length {
            v.push(self.read_type()?);
        }
        match String::from_utf8(v) {
            Ok(s) => Ok(s),
            Err(error) => Err(io::Error::new(io::ErrorKind::Other, error)),
        }
    }
}

impl<W: WriteType<u32> + WriteType<u8>> WriteType<&str> for W {
    fn write_type(&mut self, t: &str) -> io::Result<()> {
        debug_assert!(t.as_bytes().len() <= u32::MAX as usize);
        self.write_type(t.as_bytes().len() as u32)?;
        for &c in t.as_bytes() {
            self.write_type(c)?;
        }
        Ok(())
    }
}
