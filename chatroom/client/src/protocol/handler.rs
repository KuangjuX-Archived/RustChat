use std::fs;
use super::*;
use crate::MESSAGE_SIZE;

// Parse protocal by message from stdin
pub fn parse_protocol(message: &mut String){
    if message.as_str().starts_with(NMTP){
        // *message = String::from(NMTP);
        mtp_handler(message);
    }else if message.as_str().starts_with(NFTP){
        // *message = String::from(NFTP);
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

// NMTP handler function
// Split message to get client's content
// send content to sender
pub fn mtp_handler(message: &mut String){
    let s:Vec<&str> = message.split(NMTP).collect();
    *message = format!(
        "{}: {}",
        "NMTP", s[1]
    );
}

pub fn ftp_handler(message: &mut String){
    let s: Vec<&str> = message.split(NFTP).collect();
    let filename:&str = s[1];

    let contents = fs::read_to_string(filename).expect("Fail to read file!");
    if contents.len() > MESSAGE_SIZE {
        *message = String::from("Out of Buffer limit!");
    }else{
        *message = format!(
            "{}: {}: {}",
            "NFTP", String::from(filename), contents
        );
    }
    
}

// pub fn voip_handler(){

// }