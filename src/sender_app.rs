use std::{net::TcpStream, io::Write};

use crate::{client_command::ClientCommand, type_io::WriteType, message::WriteMessage};

fn send(address: &str, nickname: &str, style: u32, text: &str) {
    match TcpStream::connect(address) {
        Ok(mut stream) => {
            match stream.write_type(ClientCommand::SendMessage) {
                Ok(_) => (),
                Err(error) => {
                    println!("[Client][Sender][Error] Unable to send data: \"{error}\".");
                    return;
                },
            }

            let message = WriteMessage {
                nickname,
                style,
                text,
            };

            match stream.write_type(message) {
                Ok(_) => (),
                Err(error) => {
                    println!("[Client][Sender][Error] Unable to send data: \"{error}\".");
                    return;
                },
            }
        },
        Err(error) => println!("[Client][Sender][Error] Unable to connect: \"{error}\"."),
    }
}

pub fn run_send_client(address: &str, nickname: &str, style: u32) {
    println!("[Client][Sender] Started");
    
    let mut line = String::new();
    loop {
        line.clear();
        print!("-> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut line).unwrap();
        let text = line.replace("\n", "").replace("\r", "");
        send(address, nickname, style, text.as_str());
    }
}
