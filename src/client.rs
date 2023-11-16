use basic_serv_client::{Message, MessageType, SERVER_PORT};
use tokio::net::{TcpStream, tcp::{OwnedWriteHalf, OwnedReadHalf}};
use std::io::Write;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let (server_addr, username): (String, String) = handle_start()?;

    println!("INFO: connecting to server...");
    let stream = TcpStream::connect(format!("{}:{}", server_addr, SERVER_PORT)).await?;

    let connect_msg: Message = Message::new(
        String::new(),
        username.clone(),
        MessageType::ConnectionInit,
    );
    let data: Vec<u8> = connect_msg.into();

    stream.try_write(&data.to_owned())?;

    println!("INFO: successly connected to server. Waiting for a message...");

    let (read, write) = stream.into_split();
    handle_conversation(read, write, username).await?;

    Ok(())
}

fn handle_start() -> std::io::Result<(String, String)> {
    print!("Enter a server address: ");
    let _ = std::io::stdout().flush();

    let mut server_addr = String::new();
    std::io::stdin().read_line(&mut server_addr)?;

    print!("Enter your username: ");
    let _ = std::io::stdout().flush();
    let mut username = String::new();
    std::io::stdin().read_line(&mut username)?;

    Ok((String::from("192.168.1.22"), username))
}

async fn handle_conversation(read: OwnedReadHalf , write: OwnedWriteHalf, username: String) -> std::io::Result<()> {
    let mut content: String = String::new();
    // read
    let task = tokio::spawn(async move {
        loop {
            let mut buffer: Vec<u8> = Vec::new();
            read.readable().await.expect("An error occured while ready");
            let _ = read.try_read(&mut buffer);
            if buffer.len() > 0 {
                let recv: Message = buffer.into();
                println!("Received: {}", recv);
            }
        }
    });

    // write
    while content.trim().ne(".exit") {
        content = String::new();
        readline(&mut content);

        let msg: Vec<u8> =
            Message::new(content.clone(), username.clone(), MessageType::SendText).into();
        write.try_write(&msg.to_owned())?;
    }

    let end_msg: Vec<u8> =
        Message::new(String::new(), username, MessageType::ConnectionClosed).into();
    task.abort();
    let _ = write.try_write(&end_msg.to_owned());
    Ok(())
}

fn readline(content: &mut String) {
    print!(">> ");
    let _ = std::io::stdout().flush();
    let _ = std::io::stdin().read_line(content);
}
