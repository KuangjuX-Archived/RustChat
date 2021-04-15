use std::fs;
use std::io::prelude::*;
use chatroom::*;



// Parse protocal by message from TcpStream
pub fn parse_protocol(bytes: Vec<u8>) -> String {
    // println!("{}", message);
    // if message.as_str().starts_with(NMTP){
    //     mtp_handler(message);                      
    //     Protocol::NMTP
    // }else if message.as_str().starts_with(NFTP){
    //     ftp_handler(message);
    //     Protocol::NFTP
    // }else if message.as_str().starts_with(NVOIP){
    //     voip_handler(message);
    //     Protocol::NVoIP
    // }else if message == "help"{
    //     *message = String::from("Client view the help manual.");
    //     Protocol::Other
    // }else if message == "exit"{
    //     // If message is equivalent to : exit we'll break out of our loop
    //     *message = String::from("exit");
    //     Protocol::Other
    // }else{
    //     panic!("Error Protocol!");
    // }
    let stream = unsafe{
        Stream::deserialize(&bytes)
    };
 
    let mut message = String::new();
    match stream.protocol {
        Protocol::NMTP => {
            message = String::from_utf8(stream.contents).expect("fail to convert to string"); 
            mtp_handler(&mut message);
        },

        Protocol::NFTP => {
            message = String::from_utf8(stream.contents).expect("fail to convert to string"); 
            ftp_handler(&mut message);
        },

        Protocol::NVoIP => {
            // message = String::from("VoIP");
            voip_handler(&mut message);
        },

        _ => {
            message = String::from("Other Protocol");
        }
    }

    message

}

pub fn mtp_handler(message: &mut String){
    let s:Vec<&str> = message.split(": ").collect();
    *message = String::from(s[1]);
}

pub fn ftp_handler(message: &mut String){
    // Debug
    // println!("message: {}", message);
    let s:Vec<&str> = message.split(": ").collect();

    let name:String = String::from(s[1]);

    let s2:Vec<&str> = name.split("/").collect();
    let mut filename:String = String::from("server_files/");
    filename.push_str(s2[1]);
    let contents = s[2];

    // solve filename conflict
    duplicate_filename(&mut filename, 0);

    // let mut file = fs::File::create(filename).expect(FILE_FAIL);
    // let mut file;
    let copy_filename = filename.clone();
    match fs::File::create(filename) {
        Ok(handler) => {
            let mut file = handler;
            if file.write_all(contents.as_bytes()).is_err(){
                println!("{}", FILE_FAIL_RESPONSE);
            }
        
            println!("{}", FILE_SUCCESS);
            *message = format!(
                "{}: {}",
                copy_filename, FILE_SUCCESS_RESPONSE
            );
        }
        Err(err) => {
            println!("error: {}", err);
            *message = format!(
                "{}: {}",
                copy_filename, FILE_FAIL_RESPONSE
            );
        }
    }

}

pub fn voip_handler(message: &mut String) {
    *message = String::from("VoIP");
}
