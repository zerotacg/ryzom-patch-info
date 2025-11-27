mod header;
mod persistent_data;

use enum_ordinalize::Ordinalize;

pub use header::*;
pub use persistent_data::*;

#[derive(Debug, PartialEq, Eq, Ordinalize)]
pub enum Tokens {
    BEGIN_TOKEN,
    END_TOKEN,
    SINT_TOKEN,
    UINT_TOKEN,
    FLOAT_TOKEN,
    STRING_TOKEN,
    FLAG_TOKEN,
    EXTEND_TOKEN,
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
        Tokens::BEGIN_TOKEN => TType::STRUCT_BEGIN,
        Tokens::END_TOKEN => TType::STRUCT_END,
        Tokens::FLAG_TOKEN => TType::FLAG,
        Tokens::SINT_TOKEN if extended => TType::SINT64,
        Tokens::SINT_TOKEN => TType::SINT32,
        Tokens::UINT_TOKEN if extended => TType::UINT64,
        Tokens::UINT_TOKEN => TType::UINT32,
        Tokens::FLOAT_TOKEN if extended => TType::FLOAT64,
        Tokens::FLOAT_TOKEN => TType::FLOAT32,
        Tokens::STRING_TOKEN if extended => TType::EXTEND_TYPE,
        Tokens::STRING_TOKEN => TType::STRING,
        _ => panic!("Unknown token type"),
    }
}
