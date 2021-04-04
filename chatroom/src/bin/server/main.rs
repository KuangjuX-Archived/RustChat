mod server;
mod protocol;
use server::*;

// Localhost with a port in it
const LOCAL_HOST: &str = "127.0.0.1:8080";

// The buffer size of messages
const MESSAGE_SIZE: usize = 1024 * 1024;

use tokio::net::TcpListener;

use std::error::Error;
use std::sync::mpsc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>
{
    // let server:Server = Server::new(LOCAL_HOST);
    // let listener = server.listen();

    // // Listen IP and response
    // server.run(listener);

    let listener = TcpListener::bind(LOCAL_HOST).await?;

    // Create mutable vector for clients
    let mut clients = vec![];

    // Instantiate channel and assign it to a string type
    // We are going to be sending a bunch of strings through channel
    let (sender, receiver) = mpsc::channel::<Vec<u8>>();

    

    loop {
        let (socket, address) = listener.accept().await?;

        println!("Client {}: CONNECTED", address);
        // Clone sender
        // The socket tries to clone it and then push it to clients vector 
        // We're cloning the socket to push it into our thread 
        let sender = sender.clone();
        // clients.push(socket);

        tokio::spawn(async move {

            let mut buffer = vec![];
        });
    }

}            