mod utils;
mod logo;
pub use utils::*;
pub use logo::LOGO;

use std::ptr;


#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Protocol {
    NMTP,
    NFTP,
    NVoIP,
    Other
}

#[derive(Debug, Clone)]
#[repr(C, packed)]
pub struct Stream {
    pub protocol: Protocol,
    pub size: usize,
    pub contents: Vec<u8>
}

impl Stream {
    pub unsafe fn serialize (&self) -> Vec<u8> {
        let bytes = ::std::slice::from_raw_parts(
            (self as *const Stream) as *const u8,
            ::std::mem::size_of::<Stream>(),
        ).to_vec();

        // let my_struct = Stream::deserialize(&bytes);
        // println!("my_struct_0:{:?}", my_struct);
        println!("bytes_0:{:?}", bytes);
        bytes
    }
    
    pub unsafe fn deserialize(bytes: &Vec<u8>) -> Self {
        let (head, body, tail) = bytes.align_to::<Stream>();
        assert!(head.is_empty(), "Deserialize data fail!");
        // println!("body: {:?}", body[0].clone());
        // println!("tail: {:?}", tail);
        body[0].clone()
        
        
    }
}


// NMTP is Naive Message Transfor Protocol 
pub const NMTP: &'static str = "NMTP: ";

// NFTP is Naive File Transfor Protocol
pub const NFTP: &'static str = "NFTP: "; 


// NVoIP is Naive Voice over Internet Protoco
pub const NVOIP: &'static str = "NVoIP: "; 

// Server file response 
pub const FILE_SUCCESS: &'static str = "Succeed to download file!";
// pub const FILE_FAIL: &'static str = "Fail to download file!";

pub const FILE_SUCCESS_RESPONSE: &'static str = "Server has succeeded to receive and download file!";
pub const FILE_FAIL_RESPONSE: &'static str = "Server failed to download file!";
