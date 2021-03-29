use std::io::{self, ErrorKind, Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;
use std::str;
use super::*;
use protocol::*;

use super::*;

pub struct Client{
    ip: &'static str
}

impl Client {
    // new a client object
    pub fn new(addr: &'static str) -> Self {
        Self{
            ip: addr
        }
    }

    // comnect server by ip
    pub fn connect(&self) -> TcpStream { 
        // Create a mutable client which is a TCP stream 
        // Connect it to our local here ==> IP with the port 
        let client = TcpStream::connect(self.ip).expect("Failed to connect");
        // We want our client to be non-blocking
        // Set the flag non-blocking to true
        client.set_nonblocking(true).expect("Failed to initiate non-blocking"); 
        client
    }


    // display message in console
    pub fn display(buffer: Vec<u8>) {
        // Let message equal our buffer 
        // Turn it into an iterator 
        // Check to see if the references inside of it are equal to 0 
        // Collect all of them inside of our vector
        // All the ones that are equal to 0 are going to just discard
        let message = buffer.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
        let message = str::from_utf8(&message).unwrap();
        println!("Message :{:?}", message);
    }


    // client run!
    pub fn run(&self, mut client: TcpStream){
        // Instantiate channel and assign it to a string type
        // We are going to be sending a bunch of strings through channel
        let (sender, receiver) = mpsc::channel::<String>();

        // Spawn a thread and create a move closure inside of it with a loop
        thread::spawn(move || loop 
        {
            // Create a mutable buffer with a vector with zeros inside of it 
            let mut buffer = vec![0; MESSAGE_SIZE];
            // Read our message through the buffer
            match client.read_exact(&mut buffer) 
            {
                Ok(_) => 
                {
                    // Receive message and display in console
                    Client::display(buffer);
                },

                /* 
                * If the type of error is equal to an error that would block our non-blocking,
                * we just send back a unit type. 
                */ 
                Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),

                // If we get an error we don't care about what's inside of it 
                // We just close the connection and then we just break out of this loop
                Err(_) => 
                {
                    println!("Connection with server was severed");
                    break;
                }
            }
            match receiver.try_recv() 
            {
                Ok(message) => 
                {
                    // Clone message into bytes
                    // Put it inside of a buffer variable
                    let mut buffer = message.clone().into_bytes();
                    // Resize our buffer by our message size
                    buffer.resize(MESSAGE_SIZE, 0);
                    // Write all of our buffers into our client
                    client.write_all(&buffer).expect("Writing to socket failed");
                    // Print out the message
                    
                },
                /* 
                * Check if our try receive error is empty and 
                * if it is then we're just going to send back a unit type
                */
                Err(TryRecvError::Empty) => (),
                /*
                * Check if it's a disconnected type 
                * in which case we just want to break the loop
                */
                Err(TryRecvError::Disconnected) => break
            }
            // Have our thread sleep for a hundred milliseconds
            thread::sleep(Duration::from_millis(100));
        }); 


        // This will show up when the user opens the client
        println!("*********************************");
        println!("************ WELCOME ************");
        println!("*********************************");

        loop 
        {
            // Create a new mutable string
            let mut buffer = String::new();
            // Read into that string from our standard input
            io::stdin().read_line(&mut buffer).expect("Reading from stdin failed");
            // Trim our buffer 
            // Use the to string method to put it into a message variable 
            let mut message = buffer.trim().to_string();

            parse_protocol(&mut message);


            // If message is equivalent to : exit we'll break out of our loop
            if message == "exit" || sender.send(message).is_err(){break}

        }
        // Print out GOOD BYE
        println!("*********************************");
        println!("*********** GOOD BYE ************");
        println!("*********************************");
    }
}