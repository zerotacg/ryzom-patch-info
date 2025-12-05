use crate::pd;

pub type Token = u16;
pub type Arg = u32;

pub trait Readable {
    fn read(pdr: &mut PersistentDataRecord) -> Self;
}

pub trait ReadableProperty {
    fn read(pdr: &mut PersistentDataRecord, name: &str) -> Self;
}

impl ReadableProperty for u32 {
    fn read(pdr: &mut PersistentDataRecord, name: &str) -> Self {
        pdr.expect_token(name, pd::TType::UINT32);
        let arg = pdr.pop_arg();

        arg
    }
}

impl ReadableProperty for i32 {
    fn read(pdr: &mut PersistentDataRecord, name: &str) -> Self {
        pdr.expect_token(name, pd::TType::SINT32);
        let arg = pdr.pop_arg();

        arg as i32
    }
}

impl ReadableProperty for bool {
    fn read(pdr: &mut PersistentDataRecord, name: &str) -> Self {
        pdr.expect_token(name, pd::TType::SINT32);
        let arg = pdr.pop_arg();

        arg != 0
    }
}

impl ReadableProperty for String {
    fn read(pdr: &mut PersistentDataRecord, name: &str) -> Self {
        pdr.expect_token(name, pd::TType::STRING);
        let arg = pdr.pop_arg();

        pdr.strings[arg as usize].clone()
    }
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
    pub fn peek_token(&self) -> &pd::Tokens {
        &self.tokens[self._TokenOffset]
    }

    pub fn read_token(&mut self, name: &str) -> pd::TType {
        let token = self.pop_token(name);
        if token.value() != name {
            panic!("Expected property {} but found {}", name, token.value());
        }

        if let pd::Tokens::EXTEND_TOKEN(_) = token {
            let token = self.pop_token(name);
            if token.value() != name {
                panic!("Expected property {} but found {}", name, token.value());
            }

            return pd::token2Type(token, true);
        }

        pd::token2Type(token, false)
    }

    fn pop_token(&mut self, name: &str) -> &pd::Tokens {
        self._TokenOffset += 1;
        self.peek_token()
    }

    fn has_property(&self, name: &str) -> bool {
        name == self.peek_token().value()
    }

    fn has_begin(&self, name: &str) -> bool {
        if let pd::Tokens::BEGIN_TOKEN(_) = self.peek_token() {
            self.has_property(name)
        } else {
            false
        }
    }

    fn read_struct_begin(&mut self, name: &str) {
        self.expect_token(name, pd::TType::STRUCT_BEGIN);
    }

    fn read_struct_end(&mut self, name: &str) {
        self.expect_token(name, pd::TType::STRUCT_END);
    }

    pub fn read_prop<T: ReadableProperty>(&mut self, name: &str) -> T {
        T::read(self, name)
    }

    fn pop_arg(&mut self) -> Arg {
        let arg = self.args[self._ArgOffset];
        self._ArgOffset += 1;

        arg
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
        while self.has_begin(name) {
            items.push(self.read_struct::<T>(name));
        }

        items
    }

    pub fn read_prop_vec<T: ReadableProperty>(&mut self, name: &str) -> Vec<T> {
        let mut items: Vec<T> = Vec::new();
        while self.has_property(name) {
            items.push(self.read_prop(name));
        }

        items
    }
}
