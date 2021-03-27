mod server;
use server::*;

// Localhost with a port in it
const LOCAL_HOST: &str = "127.0.0.1:8080";

fn main()
{
    let server:Server = Server::new(LOCAL_HOST);
    let listener = server.listen();

    // Listen IP and response
    server.run(listener);
    
}            