
pub type Result<T> = core::result::Result<T, Error>;
#[derive(Debug, PartialEq, Clone)]
pub enum Error {
    Custom(String),
    InvalidSize,
    InvalidData,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Custom(s) => write!(f, "{}", s),
            Error::InvalidSize => f.write_str("Invalid size"),
            Error::InvalidData => write!(f, "Invalid data"),
        }
    }
}

impl core::error::Error for Error {
    
}