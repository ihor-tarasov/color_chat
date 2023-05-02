use std::net::TcpStream;

use crate::{client_command::ClientCommand, type_io::{WriteType, ReadType}, message::ReadMessage, draw};

fn run_recv_client_inner(address: &str, nickname: &str) {
    match TcpStream::connect(address) {
        Ok(mut stream) => {
            match stream.write_type(ClientCommand::BecomeReceiver) {
                Ok(_) => println!("[Client][Receiver] Registred as receiver."),
                Err(error) => {
                    println!("[Client][Receiver][Error] Unable to send data: \"{error}\".");
                    return;
                },
            }

            match stream.write_type(nickname) {
                Ok(_) => println!("[Client][Receiver] Sent nickname \"{nickname}\"."),
                Err(error) => {
                    println!("[Client][Receiver][Error] Unable to send data: \"{error}\".");
                    return;
                },
            }

            match <TcpStream as ReadType<bool>>::read_type(&mut stream) {
                Ok(accepted) => if !accepted {
                    println!("[Client][Receiver] Nickname \"{nickname}\" already in use.");
                    return;
                },
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
}

pub fn run_recv_client(address: &str, nickname: &str) {
    println!("[Client][Receiver] Started");
    println!("[Client][Receiver] Trying to connect to \"{address}\".");

    run_recv_client_inner(address, nickname);

    println!("[Client][Receiver] Stopped");
}
