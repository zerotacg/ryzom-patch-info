use crate::pd;

pub type Token = u16;
pub type Arg = u32;

#[derive(Debug)]
pub struct PersistentDataRecord {
    pub _TokenOffset: usize,
    pub _ArgOffset: usize,
    pub tokens: Vec<pd::Tokens>,
    pub args: Vec<Arg>,
    pub strings: Vec<String>,
}

impl PersistentDataRecord {
    pub fn peek_token(&self) -> pd::Tokens {
        self.tokens[self._TokenOffset]
    }

    pub fn read_token(&mut self) -> (pd::TType, Token) {
        let token = self.peek_token();
        self._TokenOffset += 1;

        if let pd::Tokens::EXTEND_TOKEN(token_value) = token {
            self._TokenOffset += 1;

            let token_type = pd::token2Type(token, true);
            return (token_type, token_value);
        }

        let token_type = pd::token2Type(token, false);
        (token_type, token.value())
    }

    pub fn has_struct(&self, token: Token) -> bool {
        if let pd::Tokens::BEGIN_TOKEN(token_value) = self.peek_token() {
            token == token_value
        } else {
            false
        }
    }

    pub fn read_struct_begin(&mut self) {
        let (token, token_value) = self.read_token();
        if token != pd::TType::STRUCT_BEGIN {
            panic!("Expected struct begin token");
        }
    }

    pub fn read_struct_end(&mut self) {
        let (token, token_value) = self.read_token();
        if token != pd::TType::STRUCT_END {
            panic!("Expected struct end token");
        }
    }

    pub fn read_u32(&mut self) -> u32 {
        let (token_type, token_value) = self.read_token();
        if token_type != pd::TType::UINT32 {
            panic!("Expected UINT32 token");
        }
        let arg = self.args[self._ArgOffset];
        self._ArgOffset += 1;

        arg
    }

    pub fn read_string(&mut self) -> String {
        let (token_type, token_value) = self.read_token();
        if token_type != pd::TType::STRING {
            panic!("Expected string token");
        }
        let arg = self.args[self._ArgOffset];
        self._ArgOffset += 1;

        self.strings[arg as usize].clone()
    }

    pub fn read_vec_u32(&mut self) -> Vec<u32> {
        let result = Vec::new();
        let (token_type, token_value) = self.read_token();
        if token_type != pd::TType::UINT32 {
            panic!("Expected UINT32 token");
        }
        let arg = self.args[self._ArgOffset];
        self._ArgOffset += 1;

        result
    }
}
