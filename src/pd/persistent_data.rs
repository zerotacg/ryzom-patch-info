use crate::pd;

pub type Token = u16;
pub type Arg = u32;

pub trait Readable: Sized {
    fn read(pdr: &mut PersistentDataRecord) -> Self;
}

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

    pub fn read_token(&mut self, name: &str) -> (pd::TType, Token) {
        let token = self.peek_token();
        self._TokenOffset += 1;

        if let pd::Tokens::EXTEND_TOKEN(token_value) = token {
            self._TokenOffset += 1;

            let token_type = pd::token2Type(token, true);
            return (token_type, token_value);
        }

        let token_type = pd::token2Type(token, false);
        if token.value() != self.find_token(name) {
            panic!(
                "Expected property {} but found {}",
                name,
                self.find_name(token.value())
            );
        }

        (token_type, token.value())
    }

    pub fn has_struct(&self, token: Token) -> bool {
        if let pd::Tokens::BEGIN_TOKEN(token_value) = self.peek_token() {
            token == token_value
        } else {
            false
        }
    }

    pub fn has_named_struct(&self, name: &str) -> bool {
        let token = self.find_token(name);

        self.has_struct(token)
    }

    fn find_token(&self, name: &str) -> Token {
        self.strings.iter().position(|it| *it == *name).unwrap() as Token
    }

    fn find_name(&self, token: Token) -> &String {
        &self.strings[token as usize]
    }

    pub fn read_struct_begin(&mut self, name: &str) {
        let expected_token = self.find_token(name);
        let (token_type, token_value) = self.read_token(name);
        if token_type != pd::TType::STRUCT_BEGIN {
            panic!("Expected struct begin token");
        }
        if token_value != expected_token {
            panic!(
                "Expected property {} but found {}",
                name,
                self.find_name(token_value)
            );
        }
    }

    pub fn read_struct_end(&mut self, name: &str) {
        let (token, token_value) = self.read_token(name);
        if token != pd::TType::STRUCT_END {
            panic!("Expected struct end token");
        }
    }

    pub fn read_u32(&mut self, name: &str) -> u32 {
        let (token_type, token_value) = self.read_token(name);
        if token_type != pd::TType::UINT32 {
            panic!("Expected UINT32 token");
        }
        let arg = self.args[self._ArgOffset];
        self._ArgOffset += 1;

        arg
    }

    pub fn read_string(&mut self, name: &str) -> String {
        let (token_type, token_value) = self.read_token(name);
        if token_type != pd::TType::STRING {
            panic!("Expected string token");
        }
        let arg = self.args[self._ArgOffset];
        self._ArgOffset += 1;

        self.strings[arg as usize].clone()
    }

    pub fn read_struct<T: Readable>(&mut self, name: &str) -> T {
        self.read_struct_begin(name);
        let result = T::read(self);
        self.read_struct_end(name);

        result
    }

    pub fn read_struct_vec<T: Readable>(&mut self, name: &str) -> Vec<T> {
        let mut items: Vec<T> = Vec::new();
        while self.has_named_struct(name) {
            items.push(self.read_struct::<T>(name));
        }

        items
    }
}
