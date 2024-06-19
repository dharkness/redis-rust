use crate::commands::prelude::*;

struct SetCard {
    key: String,
}

impl SetCard {
    pub fn new(key: String) -> Self {
        Self { key }
    }
}

impl Apply for SetCard {
    fn apply<'a>(&self, store: &'a mut Store) -> Result<Response<'a>, Error> {
        match store.get_if_kind(Kind::Set, &self.key) {
            IfKindResult::Matched(Value::Set(members)) => Ok(Response::Usize(members.len())),
            IfKindResult::NotSet => Ok(Response::Zero),
            _ => Err(Error::WrongType),
        }
    }
}

pub struct SetCardParser {}

impl SetCardParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl TryParse for SetCardParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, Error> {
        Ok(Box::new(SetCard::new(input.next_string()?)))
    }
}
