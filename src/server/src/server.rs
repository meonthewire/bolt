use storage::storage::{Storage, DatabaseId};

use std::{net::{TcpListener, TcpStream}, env};
use async_std::task::block_on;
use log::info;
use crate::message;

// use std::thread;

const OP_PUT: u16 = 1;
const OP_GET: u16 = 2;
const OP_DEL: u16 = 3;
const DEFAULT_HOST: &str = "127.0.0.1";
const DEFAULT_PORT: &str = "2012";

pub struct Server {
    storage: Storage,
}

impl Server {
    pub fn new() -> Self {
        Server {
            storage: Storage::new(),
        }
    }

    pub fn run(&mut self) {
        let host = env::var("BOLT_HOST").unwrap_or_else(|_| DEFAULT_HOST.to_string());
        let port_str = env::var("BOLT_PORT").unwrap_or_else(|_| DEFAULT_PORT.to_string());
        let port = port_str.parse::<u16>().expect("Invalid port number.");

        let listener = TcpListener::bind(format!("{}:{}", host, port)).expect("Error to start the server.");
        info!("BOLT is running on {}:{} ...", host, port);

        for stream in listener.incoming() {
            let stream = stream.expect("Connection error.");
            self.handle_client(stream);
        }
    }

    fn handle_client(&mut self, mut stream: TcpStream) {
        loop {
            match message::Message::receive(&mut stream) {
                Ok(message) => {
                    //println!("Received message: {:?}", message);
                    match message.code {
                        OP_PUT => {
                            let key = &message.key;
                            let value = &message.value;
                            block_on(self.storage.set(DatabaseId::Default, key, value));
                            info!("OK! PUT {} {}", key, value);
                        }
                        OP_GET => {
                            let key = &message.key;
                            let value = block_on(self.storage.get(DatabaseId::Default, key));
                            
                            match value {
                                Some(value) => {
                                    info!("OK! GET {} {}", key, value);
                                    let response = message::Message {
                                        code: OP_GET,
                                        key: key.clone(),
                                        value: value.clone(),
                                        not_found: false,
                                    };
                                    response.send(&mut stream).expect("Error sending message.");
                                }
                                None => {
                                    let response = message::Message::not_found_response();
                                    info!("ERR! KEY NOT FOUND: {}", key);
                                    response.send(&mut stream).expect("Error sending message.");
                                }
                            }
                        }
                        OP_DEL => {
                            let key = &message.key;
                            let value = block_on(self.storage.remove(DatabaseId::Default, key));
                            
                            match value {
                                Some(value) => {
                                    info!("OK! DEL {} {}", key, value);
                                    let response = message::Message {
                                        code: OP_DEL,
                                        key: key.clone(),
                                        value: value.clone(),
                                        not_found: false,
                                    };
                                    response.send(&mut stream).expect("Error sending message.");
                                }
                                None => {
                                    let response = message::Message::not_found_response();
                                    info!("ERR! KEY NOT FOUND: {}", key);
                                    response.send(&mut stream).expect("Error sending message.");
                                }
                            }
                        }
                        _ => {
                            info!("Unknown operation: {}", message.code);
                        }
                    }
                }
                Err(_) => {
                    break;
                },
            }
        }
    }
    
}
