use telnet_proj::Message;
use std::net::{TcpListener, TcpStream, Ipv4Addr};

fn main() -> std::io::Result<()> {
    let server = TcpListener::bind("0.0.0.0:6969")?;
    let addr_sub: Vec<Ipv4Addr> = Vec::new();

    for stream in server.incoming() {
        handle_connection(stream?);
    }

    Ok(())
}

fn handle_connection(stream: TcpStream) {
}

