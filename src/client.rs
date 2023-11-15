use basic_serv_client::{Message, MessageType, SERVER_PORT};
use std::io::prelude::*;
use std::net::{Ipv4Addr, TcpStream};

fn main() -> std::io::Result<()> {
    let srv_addr: String = handle_server_addr()?;

    println!("INFO: connecting to server...");
    let mut stream = TcpStream::connect(format!("192.168.1.22:{}", SERVER_PORT))?;

    let connect_msg: Message = Message::new(
        String::new(),
        String::from("username"),
        MessageType::ConnectionInit,
    );
    let data: Vec<u8> = connect_msg.into();
    println!("data: {:?}", data);
    stream.write(&data.to_owned())?;
    println!("INFO: successly connected to server. Waiting for a message...");

    Ok(())
}

fn handle_server_addr() -> std::io::Result<String> {
    print!("Enter a server address: ");
    let _ = std::io::stdout().flush();

    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer)?;

    //Ok(buffer)
    Ok(String::from("192.168.1.22"))
}
