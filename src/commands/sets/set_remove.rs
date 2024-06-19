use crate::commands::prelude::*;

struct SetRemove {
    key: String,
    values: Vec<String>,
}

impl SetRemove {
    pub fn new(key: String, values: Vec<String>) -> Self {
        Self { key, values }
    }
}

impl Apply for SetRemove {
    fn apply<'a>(&self, store: &'a mut Store) -> Result<Response<'a>, Error> {
        match store.get_mut_if_kind(Kind::Set, &self.key) {
            IfKindResult::Matched(Value::Set(ref mut members)) => {
                let mut removed = 0;
                for value in &self.values {
                    if members.remove(value) {
                        removed += 1;
                    }
                }
                if members.is_empty() {
                    store.remove(&self.key);
                }
                Ok(Response::Usize(removed))
            }
            IfKindResult::NotSet => Ok(Response::Zero),
            _ => Err(Error::WrongType),
        }
    }
}

pub struct SetRemoveParser {}

impl SetRemoveParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl TryParse for SetRemoveParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, Error> {
        Ok(Box::new(SetRemove::new(
            input.next_string()?,
            input.rest()?,
        )))
    }
}
