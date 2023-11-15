use std::net::{Ipv4Addr, TcpListener, TcpStream};
use telnet_proj::{Message, MAX_PACKET_SIZE};

fn main() -> std::io::Result<()> {
    let server = TcpListener::bind("0.0.0.0:6969")?;
    let mut addr_sub: Vec<Ipv4Addr> = Vec::new();

    for stream in server.incoming() {
        handle_connection(stream?, &mut addr_sub)?;
    }

    Ok(())
}

fn handle_connection(stream: TcpStream, addr_list: &mut Vec<Ipv4Addr>) -> std::io::Result<()> {
    println!(
        "INFO: Connection started with {}.",
        stream.peer_addr().unwrap().ip()
    );
    let mut buf: [u8; MAX_PACKET_SIZE] = [0; MAX_PACKET_SIZE];
    let length: usize = stream.peek(&mut buf)?;
    let data: Vec<u8> = buf[..length].into();

    let msg: Message = data.into();
    println!("Message received : {msg}");

    Ok(())
}
