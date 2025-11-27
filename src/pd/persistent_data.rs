use crate::pd;
use crate::pd::{TType, Tokens};
use enum_ordinalize::Ordinalize;

pub type Token = u16;
pub type Arg = u32;

#[derive(Debug)]
pub struct PersistentDataRecord {
    pub _TokenOffset: usize,
    pub _ArgOffset: usize,
    pub tokens: Vec<Token>,
    pub args: Vec<Arg>,
    pub strings: Vec<String>,
}

impl PersistentDataRecord {
    pub fn peek_token(&self) -> (pd::Tokens, Token) {
        let stored_token = self.tokens[self._TokenOffset];
        let token = pd::Tokens::from_ordinal((stored_token & 0x7) as i8);
        let token_value = stored_token >> 3;

        (token.unwrap(), token_value)
    }

    pub fn read_token(&mut self) -> (pd::TType, Token) {
        let (token, token_value) = self.peek_token();
        self._TokenOffset += 1;

        if token == pd::Tokens::EXTEND_TOKEN {
            let stored_token = self.tokens[self._TokenOffset];
            let token = pd::Tokens::from_ordinal((stored_token & 0x7) as i8);
            let token_value = stored_token >> 3;
            self._TokenOffset += 1;

            let token_type = token.map(|it| pd::token2Type(it, true));
            return (token_type.unwrap(), token_value);
        }

        let token_type = pd::token2Type(token, false);
        (token_type, token_value)
    }

    pub fn has_struct(&self, token: Token) -> bool {
        let (token_type, token_value) = self.peek_token();
        if token_type != Tokens::BEGIN_TOKEN {
            panic!("Expected struct begin token");
        }

        token == token_value
    }

    pub fn read_struct_begin(&mut self) {
        let (token, token_value) = self.read_token();
        if token != TType::STRUCT_BEGIN {
            panic!("Expected struct begin token");
        }
    }

    pub fn read_struct_end(&mut self) {
        let (token, token_value) = self.read_token();
        if token != TType::STRUCT_END {
            panic!("Expected struct end token");
        }
    }

    pub fn read_u32(&mut self) -> u32 {
        let (token_type, token_value) = self.read_token();
        if token_type != TType::UINT32 {
            panic!("Expected UINT32 token");
        }
        let arg = self.args[self._ArgOffset];
        self._ArgOffset += 1;

        arg
    }

    pub fn read_string(&mut self) -> String {
        let (token_type, token_value) = self.read_token();
        if token_type != TType::STRING {
            panic!("Expected string token");
        }
        let arg = self.args[self._ArgOffset];
        self._ArgOffset += 1;

        self.strings[arg as usize].clone()
    }

    pub fn read_vec_u32(&mut self) -> Vec<u32> {
        let result = Vec::new();
        let (token_type, token_value) = self.read_token();
        if token_type != TType::UINT32 {
            panic!("Expected UINT32 token");
        }
        let arg = self.args[self._ArgOffset];
        self._ArgOffset += 1;

        result
    }
}
