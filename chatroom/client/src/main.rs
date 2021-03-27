// Localhost with a port in it
pub const LOCAL_HOST: &str = "127.0.0.1:8080";

// The buffer size of messages
const MESSAGE_SIZE: usize = 1024;

mod client;
use client::*;


fn main() 
{
    let client = Client::new(LOCAL_HOST);
    let connect = client.connect();
    client.run(connect);

}