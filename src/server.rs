use basic_serv_client::{Message, MessageType, MAX_PACKET_SIZE, SERVER_PORT};
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let server = TcpListener::bind(format!("0.0.0.0:{}", SERVER_PORT))?;
    let username_sub: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));

    for stream in server.incoming() {
        let username_sub_clone = Arc::clone(&username_sub);
        tokio::spawn(async move {
            let _ = handle_connection(stream.expect("Failed to unwrap tcpstream"),username_sub_clone).await;
        });
    }

    Ok(())
}

async fn handle_connection(mut stream: TcpStream, subscriber: Arc<Mutex<Vec<String>>>) -> std::io::Result<()> {
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
            MessageType::ConnectionInit => {
                let mut lock = subscriber.lock().await;
                lock.push(msg.username);
                println!("INFO: users connected : {:?}", lock);
            }
            MessageType::ConnectionClosed => {
                println!("INFO: {} connection closed", msg.username);
                let mut lock = subscriber.lock().await;
                lock.retain(|username| username.ne(&msg.username));
                println!("INFO: users connected : {:?}", lock);
                break;
            }
            MessageType::SendText => {
                println!("INFO: {} send {}", msg.username, msg.content);
            }
        }
    }
    Ok(())
}
