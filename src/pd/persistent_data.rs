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

    pub fn read_token(&mut self, name: &str) -> pd::TType {
        let token = self.pop_token(name);

        if let pd::Tokens::EXTEND_TOKEN(_) = token {
            let token = self.pop_token(name);

            return pd::token2Type(token, true);
        }

        pd::token2Type(token, false)
    }

    fn pop_token(&mut self, name: &str) -> pd::Tokens {
        let expected_token = self.find_token(name);
        let token = self.peek_token();

        if token.value() != expected_token {
            panic!(
                "Expected property {} but found {}",
                name,
                self.find_name(token.value())
            );
        }

        self._TokenOffset += 1;

        token
    }

    fn has_struct(&self, name: &str) -> bool {
        let token = self.find_token(name);

        if let pd::Tokens::BEGIN_TOKEN(token_value) = self.peek_token() {
            token == token_value
        } else {
            false
        }
    }

    fn find_token(&self, name: &str) -> Token {
        self.strings.iter().position(|it| *it == *name).unwrap() as Token
    }

    fn find_name(&self, token: Token) -> &String {
        &self.strings[token as usize]
    }

    fn read_struct_begin(&mut self, name: &str) {
        self.expect_token(name, pd::TType::STRUCT_BEGIN);
    }

    fn read_struct_end(&mut self, name: &str) {
        self.expect_token(name, pd::TType::STRUCT_END);
    }

    pub fn read_u32(&mut self, name: &str) -> u32 {
        self.expect_token(name, pd::TType::UINT32);
        let arg = self.pop_arg();

        arg
    }

    pub fn read_i32(&mut self, name: &str) -> i32 {
        self.expect_token(name, pd::TType::SINT32);
        let arg = self.pop_arg();

        arg as i32
    }

    fn pop_arg(&mut self) -> Arg {
        let arg = self.args[self._ArgOffset];
        self._ArgOffset += 1;

        arg
    }

    pub fn read_string(&mut self, name: &str) -> String {
        self.expect_token(name, pd::TType::STRING);
        let arg = self.pop_arg();

        self.strings[arg as usize].clone()
    }

    fn expect_token(&mut self, name: &str, expected: pd::TType) {
        let token_type = self.read_token(name);
        if token_type != expected {
            panic!("Expected {:?} token but found {:?}", expected, token_type);
        }
    }

    pub fn read_struct<T: Readable>(&mut self, name: &str) -> T {
        self.read_struct_begin(name);
        let result = T::read(self);
        self.read_struct_end(name);

        result
    }

    pub fn read_struct_vec<T: Readable>(&mut self, name: &str) -> Vec<T> {
        let mut items: Vec<T> = Vec::new();
        while self.has_struct(name) {
            items.push(self.read_struct::<T>(name));
        }

        items
    }
}
