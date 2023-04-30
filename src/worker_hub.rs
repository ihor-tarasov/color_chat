use std::{thread::JoinHandle, sync::{Arc, Mutex, mpsc::{Sender, self}}, net::TcpStream};

use crate::{server::Server, worker::{SharedReceiver, Command, Worker}};

pub struct WorkerHub {
    joins: Vec<JoinHandle<()>>,
    server: Arc<Mutex<Server>>,
    receiver: SharedReceiver,
    sender: Sender<Command>,
}

impl WorkerHub {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        Self {
            server: Arc::new(Mutex::new(Server::new())),
            joins: Vec::new(),
            receiver,
            sender,
        }
    }

    pub fn push(&mut self) {
        let worker = Worker::new(self.joins.len(), self.receiver.clone(), self.server.clone());
        self.joins.push(std::thread::spawn(move || worker.run()))
    }

    pub fn send(&mut self, stream: TcpStream) {
        match self.sender.send(Command::Stream(stream)) {
            Ok(_) => (),
            Err(error) => println!("[Server][WorkerHub][Error] Send data to worker error: \"{error}\"."),
        }
    }
}

impl Drop for WorkerHub {
    fn drop(&mut self) {
        for _ in 0..self.joins.len() {
            match self.sender.send(Command::Terminate) {
                Ok(_) => (),
                Err(error) => println!("[Server][WorkerHub][Error] Unable to send termination command to one of workers: \"{error}\"."),
            }
        }
        while let Some(join) = self.joins.pop() {
            match join.join() {
                Ok(_) => (),
                Err(error) => println!("[Server][WorkerHub][Error] Unable to join thread of worker: \"{error:?}\"."),
            }
        }
    }
}
