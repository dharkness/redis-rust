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
    fn apply(&self, store: &mut Store) -> Result<Response, Error> {
        match store.get_if_kind(Kind::Set, &self.key) {
            IfKindResult::Matched(Value::Set(members)) => Ok(Response::Set(members.clone())),
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
