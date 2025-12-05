use crate::pd;

pub type Token = u16;
pub type Arg = u32;

pub trait Readable {
    fn read(pdr: &mut PersistentDataRecord) -> Self;
}

pub trait ReadableProperty {
    fn read(pdr: &mut PersistentDataRecord, name: &str) -> Self;
}

impl<T: Readable> ReadableProperty for T {
    fn read(pdr: &mut PersistentDataRecord, name: &str) -> Self {
        pdr.expect_token(name, pd::TType::STRUCT_BEGIN);
        let result = T::read(pdr);
        pdr.expect_token(name, pd::TType::STRUCT_END);

        result
    }
}

impl<T: ReadableProperty> ReadableProperty for Vec<T> {
    fn read(pdr: &mut PersistentDataRecord, name: &str) -> Self {
        let mut items: Vec<T> = Vec::new();
        while pdr.has_begin(name) {
            items.push(pdr.read::<T>(name));
        }

        items
    }
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

impl<T: ReadableProperty> ReadableProperty for Option<T> {
    fn read(pdr: &mut PersistentDataRecord, name: &str) -> Self {
        if (pdr.has_property(name)) {
            Some(pdr.read_prop::<T>(name))
        } else {
            None
        }
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
    fn peek_token(&self) -> &pd::Tokens {
        &self.tokens[self._TokenOffset]
    }

    fn expect_token(&mut self, name: &str, expected: pd::TType) {
        let token = self.pop_token();
        let token_type = pd::token2Type(&token, false);
        let token_value = token.value();

        if let pd::Tokens::EXTEND_TOKEN(_) = token {
            let token = self.pop_token();
            let token_type = pd::token2Type(&token, true);
            if token_type != expected || token.value() != name {
                panic!(
                    "Expected {} {:?} token but found {} {:?}",
                    name,
                    expected,
                    token.value(),
                    token_type
                );
            }
        } else if token_type != expected || token_value != name {
            panic!(
                "Expected {} {:?} token but found {} {:?}",
                name, expected, token_value, token_type
            );
        }
    }

    fn pop_token(&mut self) -> pd::Tokens {
        let value = self.peek_token().clone();

        self._TokenOffset += 1;

        value
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

    fn read_prop<T: ReadableProperty>(&mut self, name: &str) -> T {
        T::read(self, name)
    }

    fn pop_arg(&mut self) -> Arg {
        let arg = self.args[self._ArgOffset];
        self._ArgOffset += 1;

        arg
    }

    pub fn read<T: ReadableProperty>(&mut self, name: &str) -> T {
        T::read(self, name)
    }

    pub fn read_prop_vec<T: ReadableProperty>(&mut self, name: &str) -> Vec<T> {
        let mut items: Vec<T> = Vec::new();
        while self.has_property(name) {
            items.push(self.read_prop(name));
        }

        items
    }
}
