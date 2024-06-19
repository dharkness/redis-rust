use crate::commands::prelude::*;
use crate::storage::Pattern;

struct Keys {
    pattern: String,
}

impl Keys {
    pub fn new(pattern: String) -> Self {
        Self { pattern }
    }
}

impl Apply for Keys {
    fn apply<'a>(&self, store: &'a mut Store) -> Result<Response<'a>, Error> {
        if let Ok(pattern) = Pattern::try_parse(&self.pattern) {
            Ok(Response::List(store.keys(&pattern)))
        } else {
            Ok(Response::EmptyList)
        }
    }
}

pub struct KeysParser {}

impl KeysParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl TryParse for KeysParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, Error> {
        Ok(Box::new(Keys::new(input.next_string()?)))
    }
}
