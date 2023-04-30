use std::{collections::HashMap, net::TcpStream, io};

use crate::{message::WriteMessage, type_io::{WriteType, ReadType}};


pub struct Server {
    receivers: HashMap<String, TcpStream>,
    problematic_receivers: Vec<String>,
}

impl Server {
    pub fn new() -> Self {
        Self {
            receivers: HashMap::new(),
            problematic_receivers: Vec::new(),
        }
    }

    pub fn broadcast(&mut self, message: WriteMessage) {
        self.receivers.iter_mut().for_each(|(listener, stream)| {
            match stream.write_type(message.clone()) {
                Ok(_) => (),
                Err(error) => {
                    println!("[Server][Broadcast][Error] Unable to write message: \"{error}\".");
                    self.problematic_receivers.push(listener.clone());
                },
            }
        });
        while let Some(name) = self.problematic_receivers.pop() {
            self.receivers.remove(&name);
        }
    }

    pub fn new_receiver(&mut self, mut stream: TcpStream) -> io::Result<()> {
        let name = stream.read_type()?;
        self.receivers.insert(name, stream);
        Ok(())
    }
}
