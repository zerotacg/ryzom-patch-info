use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};

use clap::Parser;

pub enum PatchInfoError {
    InvalidHeader
}

#[derive(Debug)]
struct InvalidHeaderError {}

impl fmt::Display for InvalidHeaderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to parse buffer due to invalid header")
    }
}
impl Error for InvalidHeaderError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the file to read
    #[arg(short, long)]
    index_file: String,
}

fn main() -> Result<(), InvalidHeaderError> {
    let args = Args::parse();

    println!("Hello {}!", args.index_file);

    let string = args.index_file;

    read_index_file(string)?;

    Ok(())
}

fn read_index_file(string: String) -> Result<(), InvalidHeaderError> {
    let mut file = File::open(string).map_err(|_| InvalidHeaderError {})?;
    validate_header(&mut file)?;

    Ok(())
}

fn validate_header(file: &mut File) -> Result<(), InvalidHeaderError> {
    is_valid(file)
        .map_err(|_| InvalidHeaderError {})
        .and_then(|it| if it { Ok(()) } else { Err(InvalidHeaderError {}) })
}

fn is_valid(file: &mut File) -> io::Result<bool> {
    let expected_size = file.metadata().unwrap().len() as u32;
    let is_valid = expected_size > 24;

    Ok(is_valid && expected_size == read_file_size(file)?)
}

fn read_file_size(file: &mut File) -> io::Result<u32> {
    let mut buffer = [0; 4];
    file.seek(SeekFrom::Start(4))?;
    file.read_exact(&mut buffer[..])?;

    Ok(u32::from_be_bytes(buffer))
}
