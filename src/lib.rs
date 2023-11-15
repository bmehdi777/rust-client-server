use std::net::Ipv4Addr;

pub const MAX_PACKET_SIZE: usize = 65535;

#[repr(u8)]
#[derive(Debug)]
pub enum MessageType {
    ConnectionInit = 0,
    SendText = 1,
    ConnectionClosed = 2,
}

impl From<u8> for MessageType {
    fn from(value: u8) -> Self {
        match value {
            0 => return MessageType::ConnectionInit,
            1 => return MessageType::SendText,
            2 => return MessageType::ConnectionClosed,
            _ => panic!("An error occured while converting u8 into MessageType"),
        }
    }
}

pub struct Message {
    pub length: usize,
    pub message_type: MessageType,
    pub source: Ipv4Addr,
    pub content: String,
}

impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Length: {}\nMessage Type: {:?}\nSource: {}\nContent: {}\n",
            self.length, self.message_type, self.source, self.content
        )
    }
}

impl From<Vec<u8>> for Message {
    fn from(raw_bytes: Vec<u8>) -> Self {
        let length: usize = unsafe { raw_bytes[0..4].align_to::<usize>() }.1[0];
        let message_type: MessageType = raw_bytes[4].into();
        let source: Ipv4Addr =
            Ipv4Addr::new(raw_bytes[5], raw_bytes[6], raw_bytes[7], raw_bytes[8]);
        let content: String = String::from_utf8(raw_bytes[9..9 + length + 1].to_vec())
            .expect("Invalid UTF-8 sequence.");
        Message {
            length,
            message_type,
            source,
            content,
        }
    }
}

impl From<Message> for Vec<u8> {
    fn from(message: Message) -> Self {
        let src: [u8; 4] = message.source.octets();
        let content: Vec<u8> = message.content.into_bytes();
        let mut raw_bytes = vec![
            message.length as u8,
            message.message_type as u8,
            src[0],
            src[1],
            src[2],
            src[3],
        ];
        raw_bytes.extend(content);
        raw_bytes
    }
}
