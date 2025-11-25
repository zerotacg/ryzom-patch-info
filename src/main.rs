use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};

use clap::Parser;

pub enum PatchInfoError {
    InvalidHeader,
    WrongVersion,
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

type Token = u16;
type Arg = u32;

fn read_index_file(string: String) -> Result<(), InvalidHeaderError> {
    let mut file = File::open(string).map_err(|_| InvalidHeaderError {})?;
    validate_header(&mut file)?;

    let version: u32 = read_u32(&mut file).map_err(|_| InvalidHeaderError {})?;

    if version > 0 {
        println!("Version: {}", version);
        return Err(InvalidHeaderError {});
    }

    let total_size: u32 = read_u32(&mut file).map_err(|_| InvalidHeaderError {})?;
    // error if total_size!=fileSize
    let token_count: u32 = read_u32(&mut file).map_err(|_| InvalidHeaderError {})?;
    let arg_count: u32 = read_u32(&mut file).map_err(|_| InvalidHeaderError {})?;
    let string_count: u32 = read_u32(&mut file).map_err(|_| InvalidHeaderError {})?;
    let strings_size: u32 = read_u32(&mut file).map_err(|_| InvalidHeaderError {})?;
    // error if  total_size!=offset+tokenCount*sizeof(TToken)+argCount*sizeof(uint32)+stringsSize,"PDR ERROR: Invalid source data",clear();return false);
    println!("total_size: {}", total_size);
    println!("token_count: {}", token_count);
    println!("arg_count: {}", arg_count);
    println!("string_count: {}", string_count);
    println!("strings_size: {}", strings_size);

    Ok(())
}

fn validate_header(file: &mut File) -> Result<(), InvalidHeaderError> {
    is_valid(file)
        .map_err(|_| InvalidHeaderError {})
        .and_then(|it| {
            if it {
                Ok(())
            } else {
                Err(InvalidHeaderError {})
            }
        })
}

fn is_valid(file: &mut File) -> io::Result<bool> {
    let expected_size = file.metadata().unwrap().len() as u32;
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
