use crate::commands::prelude::*;

struct Len {
    key: String,
}

impl Len {
    pub fn new(key: String) -> Self {
        Self { key }
    }
}

impl Apply for Len {
    fn apply<'a>(&self, store: &'a mut Store) -> Result<Response<'a>, Error> {
        match store.get_if_kind(Kind::List, &self.key) {
            IfKindResult::Matched(Value::List(list)) => Ok(Response::Usize(list.len())),
            IfKindResult::NotSet => Ok(Response::Zero),
            _ => Err(Error::WrongType),
        }
    }
}

pub struct LenParser {}

impl LenParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl TryParse for LenParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, Error> {
        Ok(Box::new(Len::new(input.next_string()?)))
    }
}
