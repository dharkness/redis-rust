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
    fn apply<'a>(&self, store: &'a mut Store) -> Result<Response<'a>, Error> {
        match store.get_if_kind(Kind::String, &self.key) {
            IfKindResult::Matched(Value::String(s)) => Ok(Response::Usize(s.len())),
            IfKindResult::NotSet => Ok(Response::Zero),
            _ => Err(Error::WrongType),
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
