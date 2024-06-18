use crate::commands::prelude::*;

struct StrLen {
    key: String,
}

impl StrLen {
    pub fn new(key: String) -> Self {
        Self { key }
    }
}

impl Apply for StrLen {
    fn apply(&self, store: &mut Store) -> Result<Response, Error> {
        if let Some(value) = store.get(&self.key) {
            match value {
                Value::String(s) => Ok(Response::Usize(s.len())),
                _ => Err(Error::WrongType),
            }
        } else {
            Ok(Response::Zero)
        }
    }
}

pub struct StrLenParser {}

impl StrLenParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl TryParse for StrLenParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, Error> {
        Ok(Box::new(StrLen::new(input.next_string()?)))
    }
}
