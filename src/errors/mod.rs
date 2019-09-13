use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum CustomError {
    FailedToCreateOutputFile,
    FailedToWriteOutputFile,
    FailedTodownloadRemoteImage,
    FailedToCopyImageOnDisk,
    UknownImageType,
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

impl Error for CustomError {
    fn description(&self) -> &str {
        "error"
    }
}
