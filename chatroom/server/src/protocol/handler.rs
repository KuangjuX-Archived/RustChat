use std::fs;
use std::io::prelude::*;
use super::*;


// Parse protocal by message from TcpStream
pub fn parse_protocol(message: &mut String){
    if message.as_str().starts_with(NMTP){
        mtp_handler(message);
    }else if message.as_str().starts_with(NFTP){
        ftp_handler(message);
    }else if message.as_str().starts_with(NVOIP){
        *message = String::from(NVOIP);
    }else if message == "exit"{
        // If message is equivalent to : exit we'll break out of our loop
        *message = String::from("exit");
    }else{
        panic!("Error Protocol!");
    }
}

pub fn mtp_handler(message: &mut String){
    let s:Vec<&str> = message.split(NMTP).collect();
    *message = String::from(s[1]);
}

pub fn ftp_handler(message: &mut String){
    println!("message: {}", message);
    let s:Vec<&str> = message.split(": ").collect();

    let mut filename = String::from(s[1]);
    let contents = s[2];

    // solve filename conflict
    duplicate_filename(&mut filename, 0);

    let mut file = fs::File::create(filename).expect("Fail to create file!");
    
    if file.write_all(contents.as_bytes()).is_err(){
        println!("{}", FILE_FAIL_RESPONSE);
    }

    println!("{}", FILE_SUCCESS);
    *message = String::from(FILE_SUCCESS_RESPONSE);
}