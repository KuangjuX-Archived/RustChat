use std::io::{self, ErrorKind, Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;
use std::str;
use super::*;

use crate::protocol::*;

use audio::{ Audio, SAMPLE_RATE };
use chatroom::{ Protocol, LOGO };

// pub use HELP;

pub fn help() {
    println!("{}", HELP);
}

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


    // parse protocol type
    pub fn parse_protocol(message: &str) -> Protocol {
        if message.find("NVoIP") != None{
            Protocol::NVoIP
        }else if message.find("NFTP") != None{
            Protocol::NFTP
        }else{
            Protocol::NMTP
        }
    }

    /*
    * Sleep function will allow our thread to sleep for a moment.
    * Our thread will sleep for a hundred milliseconds between each of the loops.
    */    
    pub fn sleep() 
    {
        thread::sleep(::std::time::Duration::from_millis(100));
    }

    pub fn main_sleep()
    {
        thread::sleep(::std::time::Duration::from_secs(1));
    }



    // client run!
    pub fn run(&self, mut client: TcpStream){
        // Instantiate channel and assign it to a string type
        // We are going to be sending a bunch of strings through channel
        let (sender, receiver) = mpsc::channel::<Vec<u8>>();

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
                    // Let message equal our buffer 
                    // Turn it into an iterator 
                    // Check to see if the references inside of it are equal to 0 
                    // Collect all of them inside of our vector
                    // All the ones that are equal to 0 are going to just discard
                    let message = buffer.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                    let message = str::from_utf8(&message).unwrap();

                    match Client::parse_protocol(message) {
                        Protocol::NMTP => {
                            // print message
                            println!("Message :{:?}", message);
                        },

                        Protocol::NFTP => {
                            download_file(message);
                        },

                        Protocol::NVoIP => {
                            // PalyBack by default linux device
                            let mut sound = vec![0;MESSAGE_SIZE];

                            // Sleep some times for receive sound buffers
                            Client::sleep();
                            client.read_exact(&mut sound).expect("Fail to get sound");

                            // Debug
                            // println!("{:?}", sound);

                            let pcm = Audio::new_playback();
                            Audio::set_hw(&pcm);
                            let sound = Audio::u8_to_i16(&sound[..]);
                            Audio::play(&pcm, sound);

                            Client::sleep();

                            println!("Playback sound end.");
                        }, 

                        _ => {}
                    }
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
                    let mut buffer = message.clone();
                    // Resize our buffer by our message size
                    buffer.resize(MESSAGE_SIZE, 0);
                    // Write all of our buffers into our client
                    client.write_all(&buffer).expect("Writing to socket failed");
                    
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
        println!("{}", LOGO);

        loop 
        {
            // Create a new mutable string
            let mut buffer = String::new();
            // Remind user to input:
            println!("Please Input something (input 'help' to get maunual): ");
            // Read into that string from our standard input
            io::stdin().read_line(&mut buffer).expect("Reading from stdin failed");
            // Trim our buffer 
            // Use the to string method to put it into a message variable 
            let mut message = buffer.trim().to_string();

            if let Some(sound) = parse_protocol(&mut message){
                let secs = sound.len() / (SAMPLE_RATE * 2) ;
                let bytes = message.clone().into_bytes();
                if sender.send(bytes).is_err(){break}
                if sender.send(sound).is_err(){break}

                // Sleep to wait 
                // thread::sleep(::std::time::Duration::from_secs((secs + 5) as u64));

            }else {
                let bytes = message.clone().into_bytes();
                // If message is equivalent to : exit we'll break out of our loop
                if message == "exit" || sender.send(bytes.clone()).is_err(){
                    // sender.send(bytes).unwrap();
                    println!("Error message: {}", message);
                    println!("bytes: {:?}", bytes);
                    break
                }
            }

            Client::main_sleep();

        }

        // Print out GOOD BYE
        println!("Good Bye!");
        println!("{}", LOGO);
    }
}