mod error;
mod pd;

use clap::Parser;
use error::ReadingError;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Seek};

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
    let file = File::open(filepath)?;
    let file_size = file.metadata()?.len();
    if file_size < 24 {
        return Err(ReadingError::InvalidFileFormat);
    }
    let mut reader = BufReader::new(file);

    let header = read_header(file_size, &mut reader)?;
    let mut tokens: Vec<Token> = Vec::with_capacity(header.token_count as usize);
    for _ in 0..header.token_count {
        tokens.push(read_u16(&mut reader)?);
    }
    println!("Read tokens {:?}!", tokens);

    let mut args: Vec<Arg> = Vec::with_capacity(header.arg_count as usize);
    for _ in 0..header.arg_count {
        args.push(read_u32(&mut reader)?);
    }
    println!("Read args {:?}!", args);

    let mut strings: Vec<String> = Vec::with_capacity(header.string_count as usize);
    for _ in 0..header.string_count {
        strings.push(read_string(&mut reader)?);
    }
    println!("Read strings {:?}!", strings);

    Ok(header)
}

fn read_header<Stream>(size: u64, mut file: &mut Stream) -> Result<pd::Header>
where
    Stream: Read + Seek,
{
    let version: u32 = read_u32(&mut file)?;
    if version > 0 {
        return Err(ReadingError::UnsupportedVersion(version));
    }

    let total_size: u32 = read_u32(&mut file)?;
    if total_size > size as u32 {
        return Err(ReadingError::ContentTooSmall(total_size, size));
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

fn read_u16(input_stream: &mut impl Read) -> io::Result<u16> {
    let mut buffer = [0; 2];
    input_stream.read_exact(&mut buffer[..])?;

    Ok(u16::from_le_bytes(buffer))
}

fn read_u32(input_stream: &mut impl Read) -> io::Result<u32> {
    let mut buffer = [0; 4];
    input_stream.read_exact(&mut buffer[..])?;

    Ok(u32::from_le_bytes(buffer))
}

fn read_string(input_stream: &mut impl BufRead) -> io::Result<String> {
    let mut bytes = Vec::new();
    input_stream.read_until(0, &mut bytes)?;

    // remove the trailing null byte if present
    if let Some(&0) = bytes.last() {
        bytes.pop();
    }

    String::from_utf8(bytes).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}
