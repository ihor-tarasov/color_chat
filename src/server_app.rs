use std::net::TcpListener;

use crate::worker_hub::WorkerHub;

fn listen(listener: TcpListener) {
    let mut hub = WorkerHub::new();

    hub.push();
    hub.push();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => hub.send(stream),
            Err(error) => println!("[Server][Warning] Incoming connection was rejected: \"{error}\"."),
        }
    }
}

pub fn run_server(address: &str) {
    println!("[Server] Starting...");
    
    println!("[Server] Creating TcpListener for \"{address}\".");
    match TcpListener::bind(address) {
        Ok(listener) => listen(listener),
        Err(error) => println!("[Server][Error] Unable to create TcpListener: \"{error}\"."),
    };

    println!("[Server] Stoped");
}
