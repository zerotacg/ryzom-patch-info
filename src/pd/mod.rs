mod header;
mod persistent_data;

use enum_ordinalize::Ordinalize;

pub use header::*;
pub use persistent_data::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Tokens {
    BEGIN_TOKEN(String),
    END_TOKEN(String),
    SINT_TOKEN(String),
    UINT_TOKEN(String),
    FLOAT_TOKEN(String),
    STRING_TOKEN(String),
    FLAG_TOKEN(String),
    EXTEND_TOKEN(String),
}

impl Tokens {
    pub fn value(&self) -> &String {
        match self {
            Tokens::BEGIN_TOKEN(val)
            | Tokens::END_TOKEN(val)
            | Tokens::SINT_TOKEN(val)
            | Tokens::UINT_TOKEN(val)
            | Tokens::FLOAT_TOKEN(val)
            | Tokens::STRING_TOKEN(val)
            | Tokens::FLAG_TOKEN(val)
            | Tokens::EXTEND_TOKEN(val) => val,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Ordinalize)]
pub enum TType {
    STRUCT_BEGIN,
    STRUCT_END,
    FLAG,
    SINT32,
    UINT32,
    FLOAT32,
    STRING,
    SINT64,
    UINT64,
    FLOAT64,
    EXTEND_TYPE,
}

impl TType {
    pub fn is_extended(&self) -> bool {
        match self {
            TType::STRUCT_BEGIN
            | TType::STRUCT_END
            | TType::FLAG
            | TType::SINT32
            | TType::UINT32
            | TType::FLOAT32
            | TType::STRING => false,
            TType::SINT64 | TType::UINT64 | TType::FLOAT64 | TType::EXTEND_TYPE => true,
        }
    }
}

pub fn token2Type(token: &Tokens, extended: bool) -> TType {
    match token {
        Tokens::BEGIN_TOKEN(_) => TType::STRUCT_BEGIN,
        Tokens::END_TOKEN(_) => TType::STRUCT_END,
        Tokens::FLAG_TOKEN(_) => TType::FLAG,
        Tokens::SINT_TOKEN(_) if extended => TType::SINT64,
        Tokens::SINT_TOKEN(_) => TType::SINT32,
        Tokens::UINT_TOKEN(_) if extended => TType::UINT64,
        Tokens::UINT_TOKEN(_) => TType::UINT32,
        Tokens::FLOAT_TOKEN(_) if extended => TType::FLOAT64,
        Tokens::FLOAT_TOKEN(_) => TType::FLOAT32,
        Tokens::STRING_TOKEN(_) if extended => TType::EXTEND_TYPE,
        Tokens::STRING_TOKEN(_) => TType::STRING,
        _ => panic!("Unknown token type"),
    }
}
