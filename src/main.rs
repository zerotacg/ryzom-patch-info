mod error;
mod patch;
mod pd;

use clap::Parser;
use enum_ordinalize::Ordinalize;
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

    let mut pdr = read_index_file(args.index_file)?;

    let patch = patch::CProductDescriptionForClient::from(&mut pdr);
    println!("Parsed patch {:?}!", patch);

    Ok(())
}

fn read_index_file(filepath: String) -> Result<pd::PersistentDataRecord> {
    let file = File::open(filepath)?;
    let file_size = file.metadata()?.len();
    if file_size < 24 {
        return Err(ReadingError::InvalidFileFormat);
    }
    let mut reader = BufReader::new(file);

    let header = read_header(file_size, &mut reader)?;
    let mut packed_tokens: Vec<pd::Token> = Vec::with_capacity(header.token_count as usize);
    println!("Read header {:?}!", header);

    for _ in 0..header.token_count {
        packed_tokens.push(read_u16(&mut reader)?);
    }

    let mut args: Vec<pd::Arg> = Vec::with_capacity(header.arg_count as usize);
    for _ in 0..header.arg_count {
        args.push(read_u32(&mut reader)?);
    }

    let mut strings: Vec<String> = Vec::with_capacity(header.string_count as usize);
    for _ in 0..header.string_count {
        strings.push(read_string(&mut reader)?);
    }

    let tokens: Vec<pd::Tokens> = packed_tokens
        .iter()
        .map(|&x| parse_token(x, &strings))
        .collect();

    Ok(pd::PersistentDataRecord {
        _TokenOffset: 0,
        _ArgOffset: 0,
        tokens,
        args,
        strings,
    })
}

fn parse_token(stored_token: pd::Token, strings: &Vec<String>) -> pd::Tokens {
    let token_type = stored_token & 0x7;
    let token_value = stored_token >> 3;
    let token_name = strings[token_value as usize].clone();

    match token_type {
        0 => pd::Tokens::BEGIN_TOKEN(token_name),
        1 => pd::Tokens::END_TOKEN(token_name),
        2 => pd::Tokens::SINT_TOKEN(token_name),
        3 => pd::Tokens::UINT_TOKEN(token_name),
        4 => pd::Tokens::FLOAT_TOKEN(token_name),
        5 => pd::Tokens::STRING_TOKEN(token_name),
        6 => pd::Tokens::FLAG_TOKEN(token_name),
        7 => pd::Tokens::EXTEND_TOKEN(token_name),
        _ => panic!("Unknown token type"),
    }
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
        + token_count as usize * size_of::<pd::Token>()
        + arg_count as usize * size_of::<pd::Arg>()
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
