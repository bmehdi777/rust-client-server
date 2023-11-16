use basic_serv_client::{Message, MessageType, MAX_PACKET_SIZE, SERVER_PORT};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let server = TcpListener::bind(format!("0.0.0.0:{}", SERVER_PORT))?;
    let mut username_sub: Vec<String> = Vec::new();

    for stream in server.incoming() {
        tokio::spawn(async move {
            handle_connection(stream.expect("Failed to unwrap tcpstream"));
        });
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> std::io::Result<()> {
    println!(
        "INFO: Connection started with {}.",
        stream.peer_addr().unwrap().ip()
    );

    loop {
        let mut buf: [u8; MAX_PACKET_SIZE] = [0; MAX_PACKET_SIZE];
        let length: usize = stream
            .read(&mut buf)
            .expect("An error occured while reading data from network.");
        let data: Vec<u8> = buf[..length].into();

        let msg: Message = data.into();
        match msg.message_type {
            MessageType::ConnectionInit => {}
            MessageType::ConnectionClosed => {
                println!("INFO: {} connection closed", msg.username);
                break;
            }
            MessageType::SendText => {
                println!("INFO: {} send {}", msg.username, msg.content);
            }
        }
    }
    Ok(())
}
