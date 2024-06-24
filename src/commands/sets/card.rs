use crate::commands::prelude::*;

struct Card {
    key: String,
}

impl Card {
    pub fn new(key: String) -> Self {
        Self { key }
    }
}

impl Apply for Card {
    fn apply<'a>(&self, store: &'a mut Store) -> Result<Response<'a>, Error> {
        match store.get_if_kind(Kind::Set, &self.key) {
            IfKindResult::Matched(Value::Set(members)) => Ok(Response::Usize(members.len())),
            IfKindResult::NotSet => Ok(Response::Zero),
            _ => Err(Error::WrongType),
        }
    }
}

pub struct CardParser {}

impl CardParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl TryParse for CardParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, Error> {
        Ok(Box::new(Card::new(input.next_string()?)))
    }
}
