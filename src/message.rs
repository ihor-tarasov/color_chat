use std::io;

use crate::type_io::{WriteType, ReadType};


#[derive(Clone)]
pub struct WriteMessage<'a> {
    pub nickname: &'a str,
    pub style: u32,
    pub text: &'a str,
}

pub struct ReadMessage {
    pub nickname: String,
    pub style: u32,
    pub text: String,
}

impl<'a, W: WriteType<&'a str> + WriteType<u32>> WriteType<WriteMessage<'a>> for W {
    fn write_type(&mut self, t: WriteMessage<'a>) -> io::Result<()> {
        self.write_type(t.nickname)?;
        self.write_type(t.style)?;
        self.write_type(t.text)
    }
}

impl<'a, R: ReadType<String> + ReadType<u32>> ReadType<ReadMessage> for R {
    fn read_type(&mut self) -> io::Result<ReadMessage> {
        Ok(ReadMessage {
            nickname: self.read_type()?,
            style: self.read_type()?,
            text: self.read_type()?,
        })
    }
}
