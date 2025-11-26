use clap::Parser;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};

type Result<T> = std::result::Result<T, InvalidHeaderError>;

#[derive(Debug)]
enum InvalidHeaderError {
    InvalidHeader,
    UnsupportedVersion(u32),
    ContentTooSmall(u32),
    ContentWrongSize(u32, usize),
    IoError(io::Error),
}

impl fmt::Display for InvalidHeaderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            InvalidHeaderError::InvalidHeader => {
                write!(f, "Failed to parse buffer due to invalid header")
            }
            InvalidHeaderError::UnsupportedVersion(ref version) => {
                write!(f, "Unsupported version {}", version)
            }
            InvalidHeaderError::ContentTooSmall(ref total_size) => {
                write!(f, "Expected file to hold at least {} bytes", total_size)
            }
            InvalidHeaderError::ContentWrongSize(ref total_size, ref expected_size) => {
                write!(
                    f,
                    "Expected size does not add up total size should be {} bytes but got {} bytes",
                    total_size, expected_size
                )
            }
            InvalidHeaderError::IoError(ref cause) => write!(f, "Could not read file {:?}", cause),
        }
    }
}

impl Error for InvalidHeaderError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            InvalidHeaderError::InvalidHeader => None,
            InvalidHeaderError::UnsupportedVersion(..) => None,
            InvalidHeaderError::ContentTooSmall(..) => None,
            InvalidHeaderError::ContentWrongSize(..) => None,
            InvalidHeaderError::IoError(ref e) => Some(e),
        }
    }
}

impl From<io::Error> for InvalidHeaderError {
    fn from(err: io::Error) -> InvalidHeaderError {
        InvalidHeaderError::IoError(err)
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the file to read
    #[arg(short, long)]
    index_file: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    println!("Hello {}!", args.index_file);

    let string = args.index_file;

    read_index_file(string)?;

    Ok(())
}

type Token = u16;
type Arg = u32;

fn read_index_file(string: String) -> Result<()> {
    let mut file = File::open(string)?;
    let file_size = file.metadata()?.len() as u32;
    validate_header(&mut file)?;

    let version: u32 = read_u32(&mut file)?;

    if version > 0 {
        println!("Version: {}", version);
        return Err(InvalidHeaderError::UnsupportedVersion(version));
    }

    let total_size: u32 = read_u32(&mut file)?;
    if total_size > file_size {
        return Err(InvalidHeaderError::ContentTooSmall(total_size));
    }
    let token_count = read_u32(&mut file)?;
    let arg_count = read_u32(&mut file)?;
    let string_count = read_u32(&mut file)?;
    let strings_size = read_u32(&mut file)?;
    println!("total_size: {}", total_size);
    println!("token_count: {}", token_count);
    println!("arg_count: {}", arg_count);
    println!("string_count: {}", string_count);
    println!("strings_size: {}", strings_size);

    let offset = file.stream_position()?;
    let expected_size = offset as usize
        + token_count as usize * size_of::<Token>()
        + arg_count as usize * size_of::<Arg>()
        + strings_size as usize;
    if total_size as usize != expected_size {
        return Err(InvalidHeaderError::ContentWrongSize(
            total_size,
            expected_size,
        ));
    }

    Ok(())
}

fn validate_header(file: &mut File) -> Result<()> {
    if is_valid(file)? {
        Ok(())
    } else {
        Err(InvalidHeaderError::InvalidHeader)
    }
}

fn is_valid(file: &mut File) -> io::Result<bool> {
    let expected_size = file.metadata()?.len() as u32;
    let is_valid = expected_size > 24;
    let actual_size = read_file_size(file)?;
    file.seek(SeekFrom::Start(0))?;

    Ok(is_valid && expected_size == actual_size)
}

fn read_file_size(file: &mut File) -> io::Result<u32> {
    file.seek(SeekFrom::Start(4))?;

    read_u32(file)
}

fn read_u32(file: &mut File) -> io::Result<u32> {
    let mut buffer = [0; 4];
    file.read_exact(&mut buffer[..])?;

    Ok(u32::from_le_bytes(buffer))
}
