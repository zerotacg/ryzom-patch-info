mod header;
mod persistent_data;

use enum_ordinalize::Ordinalize;

pub use header::*;
pub use persistent_data::*;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Tokens {
    BEGIN_TOKEN(u16),
    END_TOKEN(u16),
    SINT_TOKEN(u16),
    UINT_TOKEN(u16),
    FLOAT_TOKEN(u16),
    STRING_TOKEN(u16),
    FLAG_TOKEN(u16),
    EXTEND_TOKEN(u16),
}

impl Tokens {
    pub fn value(&self) -> u16 {
        match self {
            Tokens::BEGIN_TOKEN(val)
            | Tokens::END_TOKEN(val)
            | Tokens::SINT_TOKEN(val)
            | Tokens::UINT_TOKEN(val)
            | Tokens::FLOAT_TOKEN(val)
            | Tokens::STRING_TOKEN(val)
            | Tokens::FLAG_TOKEN(val)
            | Tokens::EXTEND_TOKEN(val) => *val,
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
    NB_TYPE,
}

pub fn token2Type(token: Tokens, extended: bool) -> TType {
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
