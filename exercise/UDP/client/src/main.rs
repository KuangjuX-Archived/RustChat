use std::net::UdpSocket;
use std::{ io, str };

fn main() -> std::io::Result<()>{

    {
        let socket = UdpSocket::bind("127.0.0.1:12001")?;
        socket.connect("127.0.0.1:12000")?;

        let mut input = String::new();
        println!("Input something: ");
        io::stdin().read_line(&mut input)?;

        socket.send(input.as_bytes())?;

        let mut buffer = [0u8; 1024];
        socket.recv_from(&mut buffer)?;

        println!(
            "receive: {}",
            str::from_utf8(&buffer).expect("Could not write buffer as string")
        )

    }
    Ok(())
}
