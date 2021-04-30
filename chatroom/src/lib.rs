mod utils;
mod logo;
pub use utils::*;
pub use logo::LOGO;


#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Protocol {
    NMTP,
    NFTP,
    NVoIP,
    Other
}

#[repr(C, packed)]
#[derive(Debug, Clone)]
pub struct Stream {
    pub protocol: Protocol,
    pub message: Vec<u8>,
    pub size: usize
}

impl Stream {
    pub unsafe fn serialize (&self) -> Vec<u8> {
        ::std::slice::from_raw_parts(
            (self as *const Stream) as *const u8,
            ::std::mem::size_of::<Stream>(),
        ).to_vec()
    }
    
    pub unsafe fn deserialize(bytes: Vec<u8>) -> Stream {
        let (head, body, _) = bytes.align_to::<Stream>();
        assert!(head.is_empty(), "Deserialize data fail!");
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
