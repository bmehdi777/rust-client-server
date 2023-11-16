pub const MAX_PACKET_SIZE: usize = 65535;
pub const SERVER_PORT: u32 = 6969;

#[repr(u8)]
#[derive(Debug, PartialEq, Eq)]
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

#[derive(Debug)]
pub struct Message {
    pub length_content: u32,
    pub length_username: u32,
    pub message_type: MessageType,
    pub username: String,
    pub content: String,
}

impl Message {
    pub fn new(content: String, username: String, message_type: MessageType) -> Self {
        let length_content: u32 = content
            .len()
            .try_into()
            .expect("Can't parse usize into u32");

        let length_username: u32 = username
            .len()
            .try_into()
            .expect("Can't parse usize into u32");

        Message {
            length_content,
            length_username,
            message_type,
            username,
            content,
        }
    }
}

impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\nMessage Type: {:?}\nLength username: {}\nUsername: {}\nLength content: {} \nContent: {}\n",
             self.message_type, self.length_username, self.username,self.length_content, self.content
        )
    }
}

impl From<Vec<u8>> for Message {
    fn from(raw_bytes: Vec<u8>) -> Self {
        let length_content: u32 = convert_u8s_to_u32(&raw_bytes[0..4]);
        let length_username: u32 = convert_u8s_to_u32(&raw_bytes[4..8]);

        let length_content_usize: usize = length_content as usize;
        let length_username_usize: usize = length_username as usize;

        let message_type: MessageType = raw_bytes[8].into();

        let max_length_username: usize = 9 + length_username_usize;

        let username: String = String::from_utf8(raw_bytes[9..max_length_username-1].to_vec())
            .expect("Invalid UTF-8 sequence.");
        let content: String = String::from_utf8(
            raw_bytes[max_length_username..max_length_username + length_content_usize].to_vec(),
        )
        .expect("Invalid UTF-8 sequence.");

        Message {
            length_content,
            length_username,
            message_type,
            username,
            content,
        }
    }
}

impl From<Message> for Vec<u8> {
    fn from(message: Message) -> Self {
        let mut raw_bytes: Vec<u8> = Vec::new();

        let username: Vec<u8> = message.username.into_bytes();
        let content: Vec<u8> = message.content.into_bytes();

        raw_bytes.extend(message.length_content.to_ne_bytes());
        raw_bytes.extend(message.length_username.to_ne_bytes());
        raw_bytes.push(message.message_type as u8);
        raw_bytes.extend(username);
        raw_bytes.extend(content);

        raw_bytes
    }
}

fn convert_u8s_to_u32(array: &[u8]) -> u32 {
    ((array[0] as u32) << 0)
        + ((array[1] as u32) << 8)
        + ((array[2] as u32) << 16)
        + ((array[3] as u32) << 24)
}
