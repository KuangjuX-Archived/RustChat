use std::io::{self, prelude::*, BufReader, Write};
use std::net::TcpStream;
use std::str;
use std::sync::mpsc::{self, TryRecvError};
use rand::Rng;



fn main() -> std::io::Result<()> {
    let mut names:Vec<String> = Vec::new();
    names.push(String::from("Bob: "));
    names.push(String::from("Peter: "));
    names.push(String::from("John: "));
    names.push(String::from("James: "));
    names.push(String::from("KuangjuX: "));
    let mut rng = rand::thread_rng();
    let num = rng.gen_range(0..5);

    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    stream.set_nonblocking(true)?;

    let (sender, receiver) = mpsc::channel::<String>();

    let output = &mut names[num];
    loop{
        let mut input = String::new();
        println!("Please input somethings:");
        
        io::stdin()
            .read_line(&mut input)?;

        (*output).push_str(&input);

        // stream
        //     .write(output.as_bytes())
        //     .expect("Failed to write to stream");
        
        // let mut reader = BufReader::new(&stream);
        // let mut buffer: Vec<u8> = Vec::new();
        // reader
        //     .read_until(b'\n', &mut buffer)
        //     .expect("Could not read into buffer");
        // println!("{}", 
        //     str::from_utf8(&buffer).expect("Could not write buffer as string"));

        match receiver.try_recv(){
            Ok(message) => {
                let mut buffer = message.clone().into_bytes();
                buffer.resize(1024, 0);
                stream.write_all(&buffer)?;
            },
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => {
                break;
            } 
        }

        match stream.read_exact(&mut buffer) {

        }
    }

    Ok(())


    
}