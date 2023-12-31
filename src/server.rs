use basic_serv_client::{Message, MessageType, MAX_PACKET_SIZE, SERVER_PORT};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() -> std::io::Result<()> {
    let server = TcpListener::bind(format!("0.0.0.0:{}", SERVER_PORT))?;
    server
        .set_nonblocking(true)
        .expect("An error occured while setting nonblocking");
    let mut client_stream: Arc<Mutex<Vec<TcpStream>>> = Arc::new(Mutex::new(Vec::new()));
    let (tx, rx) = channel::<Message>();

    loop {
        match server.accept() {
            Ok((mut stream, _)) => {
                println!(
                    "INFO: Connection started with {}.",
                    stream.peer_addr().unwrap().ip()
                );
                client_stream
                    .lock()
                    .expect("An error occured while locking client_stream")
                    .push(stream.try_clone().expect("Can't clone the stream"));

                let tx = tx.clone();
                let client_stream = Arc::clone(&client_stream);

                thread::spawn(move || {
                    loop {
                        let mut buffer: [u8; MAX_PACKET_SIZE] = [0; MAX_PACKET_SIZE];
                        let length: usize = match stream.read(&mut buffer) {
                            Ok(l) => l,
                            Err(_) => {
                                break;
                            }
                        };
                        let data: Vec<u8> = buffer[..length].into();
                        let message: Message = data.into();
                        match message.message_type {
                            MessageType::ConnectionInit => {
                                println!("INFO: users connected : {:?}", message.username);
                            }
                            MessageType::ConnectionClosed => {
                                println!("INFO: {} connection closed", message.username);
                                client_stream
                                    .lock()
                                    .expect("client_stream poisoned.")
                                    .retain(|c| {
                                        c.peer_addr().unwrap().ne(&stream.peer_addr().unwrap())
                                    });
                                break;
                            }
                            MessageType::SendText => {
                                println!("INFO: {} send {}", message.username, message.content);
                                tx.send(message)
                                    .expect("An error occured while sending data across clients");
                            }
                        }
                    }
                    thread::sleep(std::time::Duration::from_millis(200));
                });
            }
            Err(_) => {}
        };

        if let Ok(message) = rx.try_recv() {
            println!("Data ready to send to client");
            let raw_bytes: Vec<u8> = message.into();
            let mut lock = client_stream.lock().expect("client stream poisoned.");

            for stream in lock.iter_mut() {
                println!("Sending to {:?}", stream);
                let _ = stream.write(&raw_bytes);
            }
        };
        // don't loop too quick
        thread::sleep(std::time::Duration::from_millis(500));
    }
}
