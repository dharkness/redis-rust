use crate::commands::prelude::*;

struct Get {
    key: String,
}

impl Get {
    pub fn new(key: String) -> Self {
        Self { key }
    }
}

impl Apply for Get {
    fn apply(&self, store: &mut Store) -> Result<Response, Error> {
        match store.get_if_kind(Kind::String, &self.key) {
            IfKindResult::Matched(Value::String(s)) => Ok(Response::BulkString(s.clone())),
            IfKindResult::NotSet => Ok(Response::Null),
            _ => Err(Error::WrongType),
        }
    }
}

pub struct GetParser {}

impl GetParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl TryParse for GetParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, Error> {
        Ok(Box::new(Get::new(input.next_string()?)))
    }
}
