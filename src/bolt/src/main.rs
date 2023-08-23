use std::thread;
use logger::logger;
use server::server;

fn main() {

    logger::setup_logging();
    
    let ascii_logo = r#"
   ___  ____  __ ______
  / _ )/ __ \/ //_  __/
 / _  / /_/ / /__/ /   
/____/\____/____/_/
-------------------------------------------------------------
The fucking idiot thing that stores data in key/value format.
-------------------------------------------------------------
    "#;

    println!("{}", ascii_logo);

    let mut server = server::Server::new();
    let server_handle = thread::spawn(move || {
        server.run();
    });

    // let client = client::Client::new();
    // client.run();

    server_handle.join().unwrap();
}