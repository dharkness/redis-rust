use crate::commands::prelude::*;

struct IsMember {
    key: String,
    value: String,
}

impl IsMember {
    pub fn new(key: String, value: String) -> Self {
        Self { key, value }
    }
}

impl Apply for IsMember {
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

pub struct IsMemberParser {}

impl IsMemberParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl TryParse for IsMemberParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, Error> {
        Ok(Box::new(IsMember::new(
            input.next_string()?,
            input.next_string()?,
        )))
    }
}
