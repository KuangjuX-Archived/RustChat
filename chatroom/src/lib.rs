mod utils;
pub use utils::*;

#[derive(Clone, Copy, PartialEq)]
pub enum Protocol {
    NMTP,
    NFTP,
    NVoIP,
    Other
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
