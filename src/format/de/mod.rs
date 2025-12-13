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

    /*
    #[test]
    fn test_struct() {
        #[derive(serde::Deserialize, PartialEq, Debug)]
        struct Test {
            int: u32,
            seq: Vec<String>,
        }

        let j = PersistentDataRecord {
            _TokenOffset: 0,
            _ArgOffset: 0,
            tokens: vec![
                Tokens::UINT_TOKEN("int".to_string()),
                Tokens::STRING_TOKEN("seq".to_string()),
                Tokens::STRING_TOKEN("seq".to_string()),
            ],
            args: vec![1, 2, 3],
            strings: vec![
                "int".to_string(),
                "seq".to_string(),
                "a".to_string(),
                "b".to_string(),
            ],
        };
        let expected = Test {
            int: 1,
            seq: vec!["a".to_owned(), "b".to_owned()],
        };
        assert_eq!(expected, from_pdr(&j).unwrap());
    }

    #[test]
    fn test_enum() {
        #[derive(Deserialize, PartialEq, Debug)]
        enum E {
            Unit,
            Newtype(u32),
            Tuple(u32, u32),
            Struct { a: u32 },
        }

        let j = r#""Unit""#;
        let expected = E::Unit;
        assert_eq!(expected, from_str(j).unwrap());

        let j = r#"{"Newtype":1}"#;
        let expected = E::Newtype(1);
        assert_eq!(expected, from_str(j).unwrap());

        let j = r#"{"Tuple":[1,2]}"#;
        let expected = E::Tuple(1, 2);
        assert_eq!(expected, from_str(j).unwrap());

        let j = r#"{"Struct":{"a":1}}"#;
        let expected = E::Struct { a: 1 };
        assert_eq!(expected, from_str(j).unwrap());
    }

     */
}
