mod draw;
mod styles;
mod type_io;
mod message;
mod client_command;
mod server;
mod worker;
mod worker_hub;
mod server_app;
mod receiver_app;
mod sender_app;

enum ProgramKind {
    Server,
    Receiver,
    Sender,
}

fn main() {
    let mut args = std::env::args().skip(1);

    let mut address = None;
    let mut nickname = None;
    let mut style = None;
    let mut kind = ProgramKind::Receiver;    

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "server" => kind = ProgramKind::Server,
            "sender" => kind = ProgramKind::Sender,
            _ => {
                if arg.starts_with("host=") {
                    if address.is_none() {
                        address = Some((&arg[5..]).to_string())
                    } else {
                        println!("Warning: Argument \"{arg}\" is redundant.");
                    }
                } else if arg.starts_with("nickname=") {
                    if nickname.is_none() {
                        nickname = Some((&arg[9..]).to_string())
                    } else {
                        println!("Warning: Argument \"{arg}\" is redundant.");
                    }
                } else if arg.starts_with("style=") {
                    if style.is_none() {
                        style = Some((&arg[6..]).parse::<u32>().unwrap())
                    } else {
                        println!("Warning: Argument \"{arg}\" is redundant.");
                    }
                } else {
                    println!("Warning: Unknown argument \"{arg}\".");
                }
            }
        }
    }

    let address = address.unwrap_or("127.0.0.1:9645".to_string());
    let nickname = nickname.unwrap_or("Random Man".to_string());
    let style = style.unwrap_or(0);

    match kind {
        ProgramKind::Server => server_app::run_server(address.as_str()),
        ProgramKind::Receiver => receiver_app::run_recv_client(address.as_str(), nickname.as_str()),
        ProgramKind::Sender => sender_app::run_send_client(address.as_str(), nickname.as_str(), style),
    }
}
