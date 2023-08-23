use byteordered::byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::net::TcpStream;
use std::io::Write;
use std::io::Read;

#[derive(Debug)]
pub struct Message {
    pub code: u16,
    pub key: String,
    pub value: String,
    pub not_found: bool,
}


impl Message {
    pub fn send(&self, stream: &mut TcpStream) -> std::io::Result<()> {
        let key_length = self.key.len() as u32;
        let value_length = self.value.len() as u32;

        stream.write_u16::<BigEndian>(self.code)?;
        stream.write_u32::<BigEndian>(key_length)?;
        stream.write_u32::<BigEndian>(value_length)?;
        stream.write_all(self.key.as_bytes())?;
        stream.write_all(self.value.as_bytes())?;

        Ok(())
    }

    pub fn receive(stream: &mut TcpStream) -> std::io::Result<Message> {
        let code = stream.read_u16::<BigEndian>()?;
        let key_length = stream.read_u32::<BigEndian>()?;
        let value_length = stream.read_u32::<BigEndian>()?;

        let mut key = vec![0u8; key_length as usize];
        stream.read_exact(&mut key)?;
        let key = String::from_utf8_lossy(&key).to_string();

        let mut value = vec![0u8; value_length as usize];
        stream.read_exact(&mut value)?;
        let value = String::from_utf8_lossy(&value).to_string();

        Ok(Message {
            code,
            key,
            value,
            not_found: false,
        })
    }

    pub fn not_found_response() -> Self {
        Message {
            code: 0,
            key:  String::new(),
            value: String::new(),
            not_found: true,
        }
    }
}
