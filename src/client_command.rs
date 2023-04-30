use std::io;

use crate::type_io::{ReadType, WriteType};


#[repr(u8)]
pub enum ClientCommand {
    BecomeReceiver,
    SendMessage,
}

impl<R: ReadType<u8>> ReadType<ClientCommand> for R {
    fn read_type(&mut self) -> io::Result<ClientCommand> {
        match self.read_type()? {
            0 => Ok(ClientCommand::BecomeReceiver),
            1 => Ok(ClientCommand::SendMessage),
            _ => Err(io::Error::new(io::ErrorKind::Other, "Unable to read ClientKind structure from stream.")),
        }
    }
}

impl<W: WriteType<u8>> WriteType<ClientCommand> for W {
    fn write_type(&mut self, t: ClientCommand) -> io::Result<()> {
        self.write_type(t as u8)
    }
}
