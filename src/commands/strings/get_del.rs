use crate::commands::prelude::*;

struct GetDel {
    key: String,
}

impl GetDel {
    pub fn new(key: String) -> Self {
        Self { key }
    }
}

impl Apply for GetDel {
    fn apply(&self, store: &mut Store) -> Result<Response, Error> {
        match store.get_and_remove_if_kind(Kind::String, &self.key) {
            IfKindResult::Matched(Value::String(s)) => Ok(Response::BulkString(s.clone())),
            IfKindResult::NotSet => Ok(Response::Null),
            _ => Err(Error::WrongType),
        }
    }
}

pub struct GetDelParser {}

impl GetDelParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl TryParse for GetDelParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, Error> {
        Ok(Box::new(GetDel::new(input.next_string()?)))
    }
}
