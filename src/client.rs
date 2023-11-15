use basic_serv_client::SERVER_PORT;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let srv_addr: String = handle_server_addr()?;
    println!("INFO: connecting to server...");
    TcpStream::connect(format!("192.168.1.22:{}", SERVER_PORT));

    Ok(())
}

fn handle_server_addr() -> std::io::Result<String> {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer)?;
    Ok(buffer)
}
