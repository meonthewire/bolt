use std::env;
use byteordered::byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::net::TcpStream;
use std::io::Write;
use std::io::Read;

const OP_PUT: u16 = 1;
const OP_GET: u16 = 2;
const OP_DEL: u16 = 3;
const COMMAND_PUT: &str = "put";
const COMMAND_GET: &str = "get";
const COMMAND_DEL: &str = "del";
const DEFAULT_HOST: &str = "127.0.0.1";
const DEFAULT_PORT: &str = "2012";

struct Message {
    code: u16,
    key: String,
    value: String,
}

impl Message {
    fn put(key: &str, value: &str) -> Self {
        Message {
            code: OP_PUT,
            key: key.to_string(),
            value: value.to_string(),
        }
    }

    fn get(key: &str) -> Self {
        Message {
            code: OP_GET,
            key: key.to_string(),
            value: String::new(),
        }
    }

    fn del(key: &str) -> Self {
        Message {
            code: OP_DEL,
            key: key.to_string(),
            value: String::new(),
        }
    }

    fn send(&self, stream: &mut TcpStream) -> std::io::Result<()> {
        let key_length = self.key.len() as u32;
        let value_length = self.value.len() as u32;

        stream.write_u16::<BigEndian>(self.code)?;
        stream.write_u32::<BigEndian>(key_length)?;
        stream.write_u32::<BigEndian>(value_length)?;
        stream.write_all(self.key.as_bytes())?;
        stream.write_all(self.value.as_bytes())?;

        Ok(())
    }

    fn receive(stream: &mut TcpStream) -> std::io::Result<Message> {
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
        })
    }
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    let host = env::var("BOLT_HOST").unwrap_or_else(|_| DEFAULT_HOST.to_string());
    let port_str = env::var("BOLT_PORT").unwrap_or_else(|_| DEFAULT_PORT.to_string());
    let port = port_str.parse::<u16>().expect("Invalid port number.");

    let mut stream = TcpStream::connect(format!("{}:{}", &host, &port))?;

    match command.as_str() {
        COMMAND_PUT => {
            if args.len() != 4 {
                println!("Use: bolt-ctl put <key> <val>");
                return Ok(());
            }

            let key = &args[2];
            let value: &String = &args[3];

            let message = Message::put(key, value);
            message.send(&mut stream)?;
            println!("OK! PUT {} {}", key, value);
        }
        COMMAND_GET => {
            if args.len() != 3 {
                println!("Use: bolt-ctl get <val>");
                return Ok(());
            }

            let key = &args[2];

            let message = Message::get(key);
            message.send(&mut stream)?;

            let received_message = Message::receive(&mut stream)?;
            if received_message.value.is_empty() {
                println!("ERR! KEY NOT FOUND: {}", key);
                return Ok(());
            }
            println!("OK! GET {}", received_message.value);
        }
        COMMAND_DEL => {
            if args.len() != 3 {
                println!("Use: bolt-ctl del <val>");
                return Ok(());
            }

            let key = &args[2];

            let message = Message::del(key);
            message.send(&mut stream)?;

            let received_message = Message::receive(&mut stream)?;
            if received_message.value.is_empty() {
                println!("ERR! KEY NOT FOUND: {}", key);
                return Ok(());
            }
            println!("OK! DEL {}", key);
        }
        _ => {
            println!("Unknown command: {}", command);
        }
    }

    Ok(())
}
