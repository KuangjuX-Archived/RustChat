// Localhost with a port in it
pub const LOCAL_HOST: &str = "127.0.0.1:8088";

// The buffer size of messages
pub const MESSAGE_SIZE: usize = 1024 * 1024;

// Help manual
pub const HELP: &'static str = include_str!("help.txt");

mod client;
mod protocol;

use client::*;


fn main() 
{
    let client = Client::new(LOCAL_HOST);
    loop {
        println!("Client connect");
        let connect = client.connect();
        client.run(connect);
        println!("Client exit current chatroom");
    }

}

