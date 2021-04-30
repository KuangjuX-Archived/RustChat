mod server;
mod protocol;
use server::*;
use protocol::*;

use chatroom::*;

// Localhost with a port in it
const LOCAL_HOST: &str = "127.0.0.1:8080";

// The buffer size of messages
const MESSAGE_SIZE: usize = 1024 * 1024;

use tokio::net::{ TcpListener, TcpStream };
use tokio::io::{ AsyncReadExt, AsyncWriteExt };


use std::error::Error;
use std::sync::mpsc;
use std::iter::{ IntoIterator, Iterator };
// use std::io::{ Read, Write };

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>
{
    // let server:Server = Server::new(LOCAL_HOST);
    // let listener = server.listen();

    // // Listen IP and response
    // server.run(listener);

    let listener = TcpListener::bind(LOCAL_HOST).await?;

    // Create mutable vector for clients
    let mut clients:Vec<TcpStream> = vec![];

    // Instantiate channel and assign it to a string type
    // We are going to be sending a bunch of strings through channel
    let (sender, receiver) = mpsc::channel::<Vec<u8>>();

    

    loop {
        let (mut socket, address) = listener.accept().await?;

        println!("Client {}: CONNECTED", address);
        
        // Clone sender
        // The socket tries to clone it and then push it to clients vector 
        // We're cloning the socket to push it into our thread 
        let sender = sender.clone();
        // clients.push(socket);

        tokio::spawn(async move {

            let mut buffer = vec![];

            match socket.read_exact(&mut buffer).await {
                Ok(_) => {
                    let copy = buffer.clone();
                    let (message, protocol) = Server::build_message(buffer, address);
                    let bytes = message.clone().into_bytes();

                    // Sent out message through our sender to our receiver
                    sender.send(bytes).expect("Failed to send message to receiver");

                    match protocol {
                        Protocol::NVoIP => {
                            let mut sound = vec![0; MESSAGE_SIZE];
                            socket.read_exact(&mut sound).await.expect("Fail to receive sound from client");
                            sender.send(sound).expect("Fail to send sound to receiver");

                        },

                        Protocol::NFTP => {
                            sender.send(copy).expect("Fail to send file message to receiver");
                        },

                        _ => {}
                    }
                }

                Err(_) => {

                }
            }
        });
        if let Ok(message) = receiver.try_recv() 
        {

            for i in 0..clients.len() {
                let mut buffer = message.clone();
                buffer.resize(MESSAGE_SIZE, 0);
                clients[i].write_all(&buffer).await?;
            }

            
        }


    }

}            