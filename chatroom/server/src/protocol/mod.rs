mod handler;
pub use handler::*;

pub const MESSAGE_OK_RESPONSE: &'static str = "NMTP/1.0 200\r\n Response:{}\r\n END";
pub const MESSAGE_FAIL_RESPONSE: &'static str = "NMTP/1.0 500\r\n Fail to get message\r\n END";