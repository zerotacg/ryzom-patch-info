use clap::Parser;
use error::ReadingError;
use std::fs::File;
use std::io::{self, Read, Seek};

mod error;
mod pd;

pub type Result<T> = std::result::Result<T, ReadingError>;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the file to read
    #[arg(short, long)]
    index_file: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    println!("Loading file {}!", args.index_file);

    let string = args.index_file;

    let header = read_index_file(string)?;

    println!("Read header {:?}!", header);

    Ok(())
}

type Token = u16;
type Arg = u32;

fn read_index_file(filepath: String) -> Result<pd::Header> {
    let mut file = File::open(filepath)?;

    read_header(&mut file)
}

fn read_header(mut file: &mut File) -> Result<pd::Header> {
    let file_size = file.metadata()?.len();
    if (file_size < 24) {
        return Err(ReadingError::InvalidFileFormat);
    }

    let version: u32 = read_u32(&mut file)?;
    if version > 0 {
        return Err(ReadingError::UnsupportedVersion(version));
    }

    let total_size: u32 = read_u32(&mut file)?;
    if total_size > file_size as u32 {
        return Err(ReadingError::ContentTooSmall(total_size, file_size));
    }
    let token_count = read_u32(&mut file)?;
    let arg_count = read_u32(&mut file)?;
    let string_count = read_u32(&mut file)?;
    let strings_size = read_u32(&mut file)?;

    let offset = file.stream_position()?;
    let expected_size = offset as usize
        + token_count as usize * size_of::<Token>()
        + arg_count as usize * size_of::<Arg>()
        + strings_size as usize;
    if total_size as usize != expected_size {
        return Err(ReadingError::ContentWrongSize(total_size, expected_size));
    }

    Ok(pd::Header {
        version,
        total_size,
        token_count,
        arg_count,
        string_count,
        strings_size,
    })
}

fn read_u32(file: &mut File) -> io::Result<u32> {
    let mut buffer = [0; 4];
    file.read_exact(&mut buffer[..])?;

    Ok(u32::from_le_bytes(buffer))
}
