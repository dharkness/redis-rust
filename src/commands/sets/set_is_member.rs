use crate::commands::prelude::*;

struct SetIsMember {
    key: String,
    value: String,
}

impl SetIsMember {
    pub fn new(key: String, value: String) -> Self {
        Self { key, value }
    }
}

impl Apply for SetIsMember {
    fn apply<'a>(&self, store: &'a mut Store) -> Result<Response<'a>, Error> {
        match store.get_if_kind(Kind::Set, &self.key) {
            IfKindResult::Matched(Value::Set(members)) => {
                Ok(Response::int_from_bool(members.contains(&self.value)))
            }
            IfKindResult::NotSet => Ok(Response::Zero),
            _ => Err(Error::WrongType),
        }
    }
}

pub struct SetIsMemberParser {}

impl SetIsMemberParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl TryParse for SetIsMemberParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, Error> {
        Ok(Box::new(SetIsMember::new(
            input.next_string()?,
            input.next_string()?,
        )))
    }
}
