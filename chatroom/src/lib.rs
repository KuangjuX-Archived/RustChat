mod utils;
pub use utils::*;

#[derive(Clone, Copy, PartialEq)]
pub enum Protocol {
    NMTP,
    NFTP,
    NVoIP,
    Other
}