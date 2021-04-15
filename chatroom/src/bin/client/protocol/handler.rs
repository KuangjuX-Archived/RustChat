use std::fs;
use std::io::prelude::*;
use crate::MESSAGE_SIZE;
use audio::{ SAMPLE_RATE, Audio };

use std::str::FromStr;

use crate::client::{ help, Client };
use chatroom::*;


// Parse protocal by message from stdin
pub fn parse_protocol(message: &mut String) -> Option<Vec<u8>>{
    if message.as_str().starts_with(NMTP){
        // handler Message 
        let stream = mtp_handler(message);
        let my_struct = unsafe{Stream::deserialize(&stream)};
        println!("my_struct_2: {:?}", my_struct);
        Some(stream)
        // None
    }else if message.as_str().starts_with(NFTP){
        // handler File
        let stream = ftp_handler(message);
        Some(stream)
    }else if message.as_str().starts_with(NVOIP){
        // handler Audio
        let sound = voip_handler(message);
        // return sound buffer caputered by linux device
        Some(sound)
    }else if message == "exit"{
        // If message is equivalent to : exit we'll break out of our loop
        *message = String::from("exit");
        None
    }else if message == "help" {
        help();
        None
    }else{
        // Debug
        println!("message: {}",message);
        // Invalid protocol, panic!
        panic!("Error Protocol!");
    }
}

// NMTP handler function
// Split message to get client's content
// send content to sender
pub fn mtp_handler(message: &mut String) -> Vec<u8>{
    let msg: Vec<&str> = message.split(NMTP).collect();
    let msg = format!(
        "{}", msg[1]
    );
    // println!("msg: {}", msg);
    let msg = msg.as_bytes().to_vec();
    // println!("msg: {:?}", msg);
    let stream = Stream {
        protocol: Protocol::NMTP,
        contents: msg,
        size: MESSAGE_SIZE
    };

    let bytes:Vec<u8>;
    unsafe {
        bytes = stream.serialize();
        // let my_struct = Stream::deserialize(&bytes);
        println!("bytes_1: {:?}", bytes);
        // println!("my_struct_1: {:?}", my_struct);
    }
    bytes
}

pub fn ftp_handler(message: &mut String) -> Vec<u8> {
    let s: Vec<&str> = message.split(NFTP).collect();
    let filename:&str = s[1];

    let contents = fs::read_to_string(filename).expect("Fail to read file!");
    if contents.len() > MESSAGE_SIZE {
        // *message = String::from("Out of Buffer limit!");
        panic!("File size out of buffer limit");
    }else{
        // *message = format!(
        //     "{}: {}: {}",
        //     "NFTP", String::from(filename), contents
        // );
        let msg = format!(
            "{}: {}",
            String::from(filename), contents
        );

        let msg = msg.as_bytes().to_vec();

        let stream = Stream{
            protocol: Protocol::NFTP,
            contents: msg,
            size: MESSAGE_SIZE
        };

        unsafe{
            let bytes = stream.serialize();
            bytes
        }
    }
    
}

pub fn voip_handler(message: &mut String) -> Vec<u8>{
    let s: Vec<&str> = message.split(' ').collect();
    let size = s[1];
    let size:usize = usize::from_str(size).expect("Fail to convert to usize");
    println!("size: {}", size);

    // Sleep to solve delay bugs
    Client::sleep();
    assert!(size <= 10, "Too long sound length");

    // count the size of buffer to capture sound
    let buffer_size = size * SAMPLE_RATE;

    // get linux default capture deivce
    let pcm = Audio::new_capture();

    // set hardware params in pcm
    Audio::set_hw(&pcm);

    // get sound from audio
    println!("Please input sound:");

    // capture sound by device
    let sound = Audio::capture(&pcm, buffer_size);

    // Display sound have been caputered
    println!("Sound have been caputered.");

    // Build message
    // *message = format!(
    //     "{}: {}s",
    //     "NVoIP", size
    // );

    // return a vector conveted by slice
    let sound = Audio::i16_to_u8(&sound[0..buffer_size]).to_vec();

    let stream = Stream{
        protocol: Protocol::NVoIP,
        contents: sound,
        size: MESSAGE_SIZE
    };

    unsafe {
        let bytes = stream.serialize();
        bytes
    }
}


pub fn download_file(message: &str) {
    let message = String::from(message);
    let s:Vec<&str> = message.split(":").collect();

    let name:String = String::from(s[1]);

    let s2:Vec<&str> = name.split("/").collect();
    let mut filename:String = String::from("client_files/");
    filename.push_str(s2[1]);
    let contents = s[2];

    // solve filename conflict
    duplicate_filename(&mut filename, 0);

    let copy_filename = filename.clone();
    match fs::File::create(filename) {
        Ok(handler) => {
            let mut file = handler;
            if file.write_all(contents.as_bytes()).is_err(){
                println!("Fail to download file {}", copy_filename);
            }
        
            println!("Success to download file {}", copy_filename);
        }
        Err(err) => {
            println!("error: {}", err);
        }
    }
}