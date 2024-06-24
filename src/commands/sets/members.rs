use crate::commands::prelude::*;

struct Members {
    key: String,
}

impl Members {
    pub fn new(key: String) -> Self {
        Self { key }
    }
}

impl Apply for Members {
    fn apply<'a>(&self, store: &'a mut Store) -> Result<Response<'a>, Error> {
        match store.get_if_kind(Kind::Set, &self.key) {
            IfKindResult::Matched(value) if value.is_set() => Ok(Response::ValueRef(value)),
            IfKindResult::NotSet => Ok(Response::Null),
            _ => Err(Error::WrongType),
        }
    }
}

pub struct MembersParser {}

impl MembersParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl TryParse for MembersParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, Error> {
        Ok(Box::new(Members::new(input.next_string()?)))
    }
}
