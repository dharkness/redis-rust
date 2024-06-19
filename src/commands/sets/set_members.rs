use crate::commands::prelude::*;

struct SetMembers {
    key: String,
}

impl SetMembers {
    pub fn new(key: String) -> Self {
        Self { key }
    }
}

impl Apply for SetMembers {
    fn apply<'a>(&self, store: &'a mut Store) -> Result<Response<'a>, Error> {
        match store.get_if_kind(Kind::Set, &self.key) {
            IfKindResult::Matched(value) if value.is_set() => Ok(Response::ValueRef(value)),
            IfKindResult::NotSet => Ok(Response::Null),
            _ => Err(Error::WrongType),
        }
    }
}

pub struct SetMembersParser {}

impl SetMembersParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl TryParse for SetMembersParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, Error> {
        Ok(Box::new(SetMembers::new(input.next_string()?)))
    }
}
