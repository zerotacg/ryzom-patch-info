mod child;
mod deserializer;

use crate::format::error::Result;
use crate::pd;
use deserializer::Deserializer;
use serde::Deserialize;

pub fn from_pdr<'input, T>(input: &'input pd::PersistentDataRecord) -> Result<T>
where
    T: Deserialize<'input>,
{
    let mut deserializer = Deserializer::from_pdr(input);
    let t = T::deserialize(&mut deserializer)?;

    Ok(t)
}

////////////////////////////////////////////////////////////////////////////////
mod tests {
    use super::from_pdr;
    use crate::pd::PersistentDataRecord;
    use crate::pd::Tokens;
    #[test]
    fn test_simple_values() {
        #[derive(serde::Deserialize, PartialEq, Debug)]
        struct Test {
            int: u32,
            a: String,
            b: String,
        }

        let j = PersistentDataRecord {
            _TokenOffset: 0,
            _ArgOffset: 0,
            tokens: vec![
                Tokens::UINT_TOKEN("int".to_string()),
                Tokens::STRING_TOKEN("a".to_string()),
                Tokens::STRING_TOKEN("b".to_string()),
            ],
            args: vec![1, 1, 2],
            strings: vec!["int".to_string(), "a".to_string(), "b".to_string()],
        };
        let expected = Test {
            int: 1,
            a: "a".to_owned(),
            b: "b".to_owned(),
        };
        assert_eq!(expected, from_pdr(&j).unwrap());
    }

    #[test]
    fn test_sequence() {
        #[derive(serde::Deserialize, PartialEq, Debug)]
        struct Test {
            seq: Vec<u32>,
        }

        let j = PersistentDataRecord {
            _TokenOffset: 0,
            _ArgOffset: 0,
            tokens: vec![
                Tokens::UINT_TOKEN("seq".to_string()),
                Tokens::UINT_TOKEN("seq".to_string()),
                Tokens::UINT_TOKEN("seq".to_string()),
            ],
            args: vec![1, 2, 3],
            strings: vec!["seq".to_string()],
        };
        let expected = Test { seq: vec![1, 2, 3] };
        assert_eq!(expected, from_pdr(&j).unwrap());
    }
}
