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
        if self.receivers.contains_key(&name) {
            stream.write_type(false)?;
            self.broadcast(WriteMessage { nickname: "Server", style: 0, text: format!("Somebody trying to connect as a receiver using nickname \"{name}\", but this nickname already used. Connection rejected.").as_str() })
        } else {
            stream.write_type(true)?;
            self.broadcast(WriteMessage { nickname: "Server", style: 0, text: format!("Receiver \"{name}\" connected.").as_str() });
            self.receivers.insert(name, stream);
        }
        Ok(())
    }
}
