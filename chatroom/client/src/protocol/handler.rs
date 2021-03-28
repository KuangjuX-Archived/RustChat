use std::fs;
use super::*;
use crate::MESSAGE_SIZE;

// NMTP handler function
// Split message to get client's content
// send content to sender
pub fn mtp_handler(message: &mut String){
    let s:Vec<&str> = message.split(NMTP).collect();
    *message = String::from(s[1]);
}

pub fn ftp_handler(message: &mut String){
    let s: Vec<&str> = message.split(NFTP).collect();
    let filename:&str = s[1];

    let contents = fs::read_to_string(filename).expect("Fail to read file!");
    if contents.len() > MESSAGE_SIZE {
        *message = String::from("Out of Buffer limit!");
    }
    *message = contents;
}

pub fn voip_handler(){

}