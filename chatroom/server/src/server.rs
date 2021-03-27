use std::io::{ErrorKind, Read, Write};
use std::net::{ TcpListener, SocketAddr };
use std::sync::mpsc;
use std::thread;

// The buffer size of messages
const MESSAGE_SIZE: usize = 1024;



pub struct Server{
    pub host: &'static str
}

impl Server{
    pub fn new(ip: &'static str) -> Self{
        Self{
            host: ip
        }
    }

    pub fn listen(&self) -> TcpListener{
        // Instantiate server 
        let listener = TcpListener::bind(self.host).expect("Failed Bind!");
        // Push listener in non-blocking mode
        listener.set_nonblocking(true).expect("Fail to set nonblocking!");
        listener
        
    }

    /*
    * Sleep function will allow our thread to sleep for a moment.
    * Our thread will sleep for a hundred milliseconds between each of the loops.
    */    
    fn sleep() 
    {
        thread::sleep(::std::time::Duration::from_millis(100));
    }

    
    
    fn build_message(buffer: Vec<u8>, address: SocketAddr) -> String{
        // Take the message that we're receiving 
        // Convert it into an iterator 
        // Take all the characters that are not whitespaces
        // Collect them inside of out vector 
        let message = buffer.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
        // Convert slice of strings into an actual string 
        let message = String::from_utf8(message).expect("Invalid utf8 message");
        // Print out the address sent the message
        println!("{}: {:?}", address, message);

        // Concatenate address with message
        let message = format!(
            "{}: {}",
            address,
            message
        );

        message
    }

    pub fn run(&self, listener: TcpListener){
        // Create mutable vector for clients
        let mut clients = vec![];

        // Instantiate channel and assign it to a string type
        // We are going to be sending a bunch of strings through channel
        let (sender, receiver) = mpsc::channel::<String>();
        
        loop 
        {   
            // Destruct result from listener.accept()  
            // listener.accept() allows us to accept connections to this server
            // socket: TCP stream 
            // address: socket address 
            if let Ok((mut socket, address)) = listener.accept()
            {
                println!("Client {}: CONNECTED", address);
                // Clone sender
                // The socket tries to clone it and then push it to clients vector 
                // We're cloning the socket to push it into our thread 
                let sender = sender.clone();
                clients.push(socket.try_clone().expect("Failed to clone client"));
                
                // Spawn our thread here with a move closure inside of it 
                thread::spawn(move || loop 
                {
                    // Create a mutable buffer 
                    let mut buffer = vec![0; MESSAGE_SIZE];
                    // Read our message into our buffer 
                    match socket.read_exact(&mut buffer) 
                    {
                        Ok(_) => {
                            let message = Server::build_message(buffer, address);
                            // Sent out message through our sender to our receiver
                            sender.send(message).expect("Failed to send message to receiver");
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
                            println!("Closing connection with: {}", address);
                            break;
                        } 
                    }
                    /*
                    * Our thread would be constantly looping around and it would be really awkward. 
                    * Sleep function will allow our loop to sort of rest while it's not receiving messages.
                    */   
                    Server::sleep();
                }); 
            }      
            if let Ok(message) = receiver.try_recv() 
            {
                clients = clients.into_iter().filter_map(|mut client| 
                {
                    // Set the buffer equal to message that clone into bytes 
                    // Convert our messages into bytes
                    let mut buffer = message.clone().into_bytes();
                    // Resize buffer based on our message size 
                    buffer.resize(MESSAGE_SIZE, 0);
                    // Take our client 
                    // Write all of the entire buffer 
                    // Map it into our client
                    // Send it back 
                    // Collect it all into a vector
                    client.write_all(&buffer).map(|_| client).ok()
                }).collect::<Vec<_>>();
            }
                Server::sleep();
        }
    }
}