use std::net::TcpStream;

use crate::{client_command::ClientCommand, type_io::{WriteType, ReadType}, message::ReadMessage, draw};


pub fn run_recv_client(address: &str, nickname: &str) {
    println!("[Client][Receiver] Started");

    match TcpStream::connect(address) {
        Ok(mut stream) => {
            match stream.write_type(ClientCommand::BecomeReceiver) {
                Ok(_) => (),
                Err(error) => {
                    println!("[Client][Receiver][Error] Unable to send data: \"{error}\".");
                    return;
                },
            }

            match stream.write_type(nickname) {
                Ok(_) => (),
                Err(error) => {
                    println!("[Client][Receiver][Error] Unable to send data: \"{error}\".");
                    return;
                },
            }

            loop {
                let message: ReadMessage = match stream.read_type() {
                    Ok(message) => message,
                    Err(error) => {
                        println!("[Client][Receiver][Error] Unable to receive data: \"{error}\".");
                        break
                    },
                };

                draw::frame(message.style as usize, message.nickname.as_str(), message.text.as_str());
            }
        },
        Err(error) => println!("[Client][Sender][Error] Unable to connect: \"{error}\"."),
    }

    println!("[Client][Receiver] Stopped");
}
