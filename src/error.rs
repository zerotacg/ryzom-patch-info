use std::error::Error;
use std::{fmt, io};

#[derive(Debug)]
pub enum ReadingError {
    InvalidFileFormat,
    UnsupportedVersion(u32),
    ContentTooSmall(u32, u64),
    ContentWrongSize(u32, usize),
    IoError(io::Error),
}

impl fmt::Display for ReadingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ReadingError::InvalidFileFormat => {
                write!(f, "Invalid File Format")
            }
            ReadingError::UnsupportedVersion(ref version) => {
                write!(f, "Unsupported version {}", version)
            }
            ReadingError::ContentTooSmall(ref total_size, ref file_size) => {
                write!(
                    f,
                    "Expected file to hold at least {} bytes but contains only {} bytes ",
                    total_size, file_size
                )
            }
            ReadingError::ContentWrongSize(ref total_size, ref expected_size) => {
                write!(
                    f,
                    "Expected size does not add up total size should be {} bytes but got {} bytes",
                    total_size, expected_size
                )
            }
            ReadingError::IoError(ref cause) => write!(f, "Could not read file {:?}", cause),
        }
    }
}

impl Error for ReadingError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            ReadingError::InvalidFileFormat => None,
            ReadingError::UnsupportedVersion(..) => None,
            ReadingError::ContentTooSmall(..) => None,
            ReadingError::ContentWrongSize(..) => None,
            ReadingError::IoError(ref e) => Some(e),
        }
    }
}

impl From<io::Error> for ReadingError {
    fn from(err: io::Error) -> ReadingError {
        ReadingError::IoError(err)
    }
}
