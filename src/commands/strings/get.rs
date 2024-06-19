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
    fn apply<'a>(&self, store: &'a mut Store) -> Result<Response<'a>, Error> {
        match store.get_if_kind(Kind::String, &self.key) {
            IfKindResult::Matched(value) if value.is_string() => Ok(Response::ValueRef(value)),
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
