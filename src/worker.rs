use std::{net::TcpStream, sync::{Arc, Mutex, mpsc::Receiver}, io};

use crate::{server::Server, client_command::ClientCommand, type_io::ReadType, message::{ReadMessage, WriteMessage}};

pub enum Command {
    Stream(TcpStream),
    Terminate,
}

pub type SharedReceiver = Arc<Mutex<Receiver<Command>>>;

pub struct Worker {
    id: usize,
    receiver: SharedReceiver,
    server: Arc<Mutex<Server>>,
}

impl Worker {
    pub fn new(id: usize, receiver: SharedReceiver, server: Arc<Mutex<Server>>) -> Self {
        Self { id, receiver, server }
    }

    fn process(&self, mut stream: TcpStream) -> io::Result<()> {
        let command: ClientCommand = stream.read_type()?;

        match command {
            ClientCommand::BecomeReceiver => match self.server.lock() {
                Ok(mut server) => server.new_receiver(stream)?,
                Err(error) => return Err(io::Error::new(io::ErrorKind::Other, error.to_string())),
            }
            ClientCommand::SendMessage => {
                let message: ReadMessage = stream.read_type()?;

                let write_message = WriteMessage {
                    nickname: message.nickname.as_str(),
                    style: message.style,
                    text: message.text.as_str(),
                };

                match self.server.lock() {
                    Ok(mut server) => server.broadcast(write_message),
                    Err(error) => return Err(io::Error::new(io::ErrorKind::Other, error.to_string())),
                }
            },
        }

        Ok(())
    }

    pub fn run(&self) {
        println!("[Server][Worker#{}] Started.", self.id);
        loop {
            let stream = match self.receiver.lock() {
                Ok(receiver) => match receiver.recv() {
                    Ok(command) => match command {
                        Command::Stream(stream) => stream,
                        Command::Terminate => break,
                    },
                    Err(error) => {
                        println!("[Server][Worler#{}][Warning] Unable to receive data: \"{}\".", self.id, error);
                        continue;
                    },
                },
                Err(error) => {
                    println!("[Server][Worler#{}][Error] Unable to lock receiver: \"{}\".", self.id, error);
                    break;
                },
            };

            match self.process(stream) {
                Ok(_) => (),
                Err(error) => println!("[Server][Worler#{}][Warning] Client processiong error: \"{}\".", self.id, error),
            }
        }
        println!("[Server][Worker#{}] Stopped.", self.id);
    }
}
