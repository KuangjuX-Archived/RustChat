mod handler;
mod utils;
pub use handler::*;
pub use utils::*;


// NMTP is Naive Message Transfor Protocol 
pub const NMTP: &'static str = "NMTP: ";

// NFTP is Naive File Transfor Protocol
pub const NFTP: &'static str = "NFTP: "; 

// Server file response 
pub const FILE_SUCCESS: &'static str = "Succeed to download file!";
pub const FILE_FAIL: &'static str = "Fail to download file!";

pub const FILE_SUCCESS_RESPONSE: &'static str = "Server has succeeded to receive and download file!";
pub const FILE_FAIL_RESPONSE: &'static str = "Server failed to download file!";

// NVoIP is Naive Voice over Internet Protoco
pub const NVOIP: &'static str = "NVoIP: "; 
