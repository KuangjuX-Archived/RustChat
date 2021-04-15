mod server;
mod protocol;
use server::*;

use chatroom::*;



// Localhost with a port in it
const LOCAL_HOST: &str = "127.0.0.1:8088";

// The buffer size of messages
const MESSAGE_SIZE: usize = 1024 * 1024;



fn main()
{
    let server:Server = Server::new(LOCAL_HOST);
    let listener = server.listen();

    // Listen IP and response
    server.run(listener);

    // let test:Vec<u8> = vec![1,2,3,4,5];

    // let stream = Stream {
    //     protocol: Protocol::NMTP,
    //     content: test,
    //     size: 0
    // };

    // let bytes = unsafe{stream.serialize()};

    // let my_struct = unsafe{Stream::deserialize(bytes)};

    // println!("{:?}", my_struct);
}           