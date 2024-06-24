use crate::commands::prelude::*;

struct Remove {
    key: String,
    values: Vec<String>,
}

impl Remove {
    pub fn new(key: String, values: Vec<String>) -> Self {
        Self { key, values }
    }
}

impl Apply for Remove {
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

pub struct RemoveParser {}

impl RemoveParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl TryParse for RemoveParser {
    fn try_parse(&self, input: &mut Input) -> Result<Box<dyn Apply>, Error> {
        Ok(Box::new(Remove::new(input.next_string()?, input.rest()?)))
    }
}
